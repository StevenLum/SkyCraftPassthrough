//! Mesh composition before Skyrim's HUD. No Skyrim target is kept between frames.
use std::{cell::RefCell,ffi::c_void,io,ptr,sync::OnceLock,time::Instant};
use ::windows::{core::{Interface,PCSTR,s},Win32::Graphics::{Direct3D::*,Direct3D::Fxc::D3DCompile,Direct3D11::*,Dxgi::Common::*}};
use crate::{Role,native::write_log,scene::{self,Mailbox,Packet,HEADER},windows::now_ms};
struct Api { renderer:usize,camera:usize }
static API:OnceLock<Api>=OnceLock::new();
pub fn configure(renderer:usize,camera:usize) { let _=API.set(Api{renderer,camera}); }
#[derive(Default)]
struct State { renderer:Option<Renderer>,failed:bool }
thread_local! { static STATE:RefCell<State>=RefCell::new(State::default()); }

// Caller has already swapped into the overlay context state. Errors affect only this pass.
pub unsafe fn draw(device:&ID3D11Device,context:&ID3D11DeviceContext1,target:&ID3D11RenderTargetView,width:u32,height:u32) {
    let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||STATE.with(|state|->io::Result<()> {
        let mut state=state.borrow_mut();
        if state.failed || scene::alignment().is_none() || API.get().is_none() { return Ok(()); }
        if state.renderer.is_none() { state.renderer=Some(unsafe { Renderer::new(device) }?); }
        unsafe { state.renderer.as_mut().unwrap().frame(device,context,target,width,height) }
    })));
    unsafe { context.PSSetShaderResources(0,Some(&[None,None])); context.OMSetRenderTargets(None,None); }
    if !matches!(outcome,Ok(Ok(()))) {
        let message=match outcome { Ok(Err(e))=>e.to_string(),_=>"world renderer panic".into() };
        let _=write_log(Role::Skyrim,format!("event=SCENE_DISABLED message={message}"));
        STATE.with(|s| { let mut s=s.borrow_mut(); s.failed=true; s.renderer=None; });
    }
}
unsafe fn read<T:Copy>(base:usize,offset:usize)->T { unsafe { ptr::read_unaligned((base+offset) as *const T) } }
fn err(e: ::windows::core::Error)->io::Error { io::Error::other(e) }

const SHADER:&[u8]=br#"
cbuffer Frame : register(b0) {
    row_major float4x4 viewProj;
    float4 origin;
    float4 depthParams; // near, far, Skyrim reversed Z, pass: 0 solid / 1 cracks / 2 translucent
    float4 screen; // width, height, reserved
};
Texture2D atlas : register(t0);
Texture2D<float> sceneDepth : register(t1);
SamplerState nearest : register(s0);
struct Input { float3 pos:POSITION; float2 uv:TEXCOORD0; float4 color:COLOR0; };
struct Vertex { float4 pos:SV_Position; float2 uv:TEXCOORD0; float4 color:COLOR0; float viewZ:TEXCOORD1; };
Vertex vs(Input i) {
    Vertex o; float3 rel=origin.xyz+float3(i.pos.x,-i.pos.z,i.pos.y)*70;
    o.pos=mul(viewProj,float4(rel,1)); o.viewZ=o.pos.w; o.uv=i.uv; o.color=i.color; return o;
}
float4 ps(Vertex i):SV_Target {
    uint w,h; sceneDepth.GetDimensions(w,h);
    int2 p=min(int2(i.pos.xy/screen.xy*float2(w,h)),int2(w-1,h-1));
    float d=sceneDepth.Load(int3(p,0)); float n=depthParams.x,f=depthParams.y;
    float skyZ=depthParams.z>0.5 ? n*f/(n+d*(f-n)) : n*f/(f-d*(f-n));
    // Small tolerance avoids flicker where imported surfaces touch Skyrim's geometry.
    if(i.viewZ>skyZ+1.0) discard;
    float4 c=atlas.Sample(nearest,i.uv)*i.color;
    if(depthParams.w>1.5) return c;
    // Crack textures multiply the block's color: white leaves it alone, black
    // darkens it. Alpha blending black with this opacity gives the same effect
    // for Minecraft's grayscale cracks, including transparent texture pixels.
    if(depthParams.w>0.5) return float4(0,0,0,c.a*(1-dot(c.rgb,float3(0.333333,0.333333,0.333333))));
    clip(c.a-0.5); return float4(c.rgb,1);
}
float4 probeVs(uint id:SV_VertexID):SV_Position {
    float2 uv=float2((id<<1)&2,id&2); return float4(uv*float2(2,-2)+float2(-1,1),0,1);
}
float probePs(float4 pos:SV_Position):SV_Target {
    uint w,h; sceneDepth.GetDimensions(w,h); uint k=(uint)pos.x;
    // Add a crosshair sample without changing the original 16-point depth grid.
    float2 uv=k==16 ? float2(0.5,0.5) : (float2(k%4,k/4)+0.5)/4.0;
    return sceneDepth.Load(int3(min(uint2(uv*float2(w,h)),uint2(w-1,h-1)),0));
}
"#;
unsafe fn compile(entry:PCSTR,target:PCSTR)->io::Result<Vec<u8>> { unsafe {
    let mut code=None; let mut errors=None;
    let result=D3DCompile(SHADER.as_ptr().cast(),SHADER.len(),s!("PassthroughWorld"),None,None,entry,target,0,0,&mut code,Some(&mut errors));
    if let Some(errors)=errors {
        let bytes=std::slice::from_raw_parts(errors.GetBufferPointer().cast::<u8>(),errors.GetBufferSize());
        let _=write_log(Role::Skyrim,format!("event=SCENE_SHADER message={}",String::from_utf8_lossy(bytes).replace(['\r','\n']," ")));
    }
    result.map_err(err)?; let code=code.unwrap();
    Ok(std::slice::from_raw_parts(code.GetBufferPointer().cast::<u8>(),code.GetBufferSize()).to_vec())
} }

struct Renderer {
    mesh_box:Mailbox,atlas_box:Mailbox,mesh:Option<Packet>,atlas:Option<Packet>,
    vertices:Option<ID3D11Buffer>,texture:Option<ID3D11ShaderResourceView>,
    vs:ID3D11VertexShader,ps:ID3D11PixelShader,layout:ID3D11InputLayout,constants:ID3D11Buffer,
    raster:ID3D11RasterizerState,sampler:ID3D11SamplerState,normal:ID3D11DepthStencilState,reversed:ID3D11DepthStencilState,
    crack_blend:ID3D11BlendState,crack_normal:ID3D11DepthStencilState,crack_reversed:ID3D11DepthStencilState,
    depth:Option<ID3D11DepthStencilView>,size:(u32,u32),
    probe_vs:ID3D11VertexShader,probe_ps:ID3D11PixelShader,probe_tex:ID3D11Texture2D,
    probe_rtv:ID3D11RenderTargetView,staging:ID3D11Texture2D,pending:bool,last_probe:u64,
    convention:Option<bool>,source:usize,last_log:u64,drawn:Option<(u64,u64)>,
}
impl Renderer {
    unsafe fn new(device:&ID3D11Device)->io::Result<Self> { unsafe {
        let code=compile(s!("vs"),s!("vs_5_0"))?;
        let mut vs=None; let mut ps=None; let mut layout=None; let mut constants=None;
        device.CreateVertexShader(&code,None,Some(&mut vs)).map_err(err)?;
        device.CreatePixelShader(&compile(s!("ps"),s!("ps_5_0"))?,None,Some(&mut ps)).map_err(err)?;
        let element=|name,format,offset|D3D11_INPUT_ELEMENT_DESC{SemanticName:name,Format:format,AlignedByteOffset:offset,InputSlotClass:D3D11_INPUT_PER_VERTEX_DATA,..Default::default()};
        device.CreateInputLayout(&[element(s!("POSITION"),DXGI_FORMAT_R32G32B32_FLOAT,0),element(s!("TEXCOORD"),DXGI_FORMAT_R32G32_FLOAT,12),element(s!("COLOR"),DXGI_FORMAT_R8G8B8A8_UNORM,20)],&code,Some(&mut layout)).map_err(err)?;
        device.CreateBuffer(&D3D11_BUFFER_DESC { ByteWidth:112,Usage:D3D11_USAGE_DEFAULT,BindFlags:D3D11_BIND_CONSTANT_BUFFER.0 as u32,..Default::default() },None,Some(&mut constants)).map_err(err)?;
        let mut raster=None; let mut sampler=None;
        device.CreateRasterizerState(&D3D11_RASTERIZER_DESC { FillMode:D3D11_FILL_SOLID,CullMode:D3D11_CULL_NONE,DepthClipEnable:true.into(),..Default::default() },Some(&mut raster)).map_err(err)?;
        device.CreateSamplerState(&D3D11_SAMPLER_DESC { Filter:D3D11_FILTER_MIN_MAG_MIP_POINT,AddressU:D3D11_TEXTURE_ADDRESS_CLAMP,AddressV:D3D11_TEXTURE_ADDRESS_CLAMP,AddressW:D3D11_TEXTURE_ADDRESS_CLAMP,ComparisonFunc:D3D11_COMPARISON_NEVER,MaxAnisotropy:1,MaxLOD:0.0,..Default::default() },Some(&mut sampler)).map_err(err)?;
        let stencil=D3D11_DEPTH_STENCILOP_DESC { StencilFailOp:D3D11_STENCIL_OP_KEEP,StencilDepthFailOp:D3D11_STENCIL_OP_KEEP,StencilPassOp:D3D11_STENCIL_OP_KEEP,StencilFunc:D3D11_COMPARISON_ALWAYS };
        let mut normal=None; let mut reversed=None;
        for (func,out) in [(D3D11_COMPARISON_LESS_EQUAL,&mut normal),(D3D11_COMPARISON_GREATER_EQUAL,&mut reversed)] {
            device.CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC { DepthEnable:true.into(),DepthWriteMask:D3D11_DEPTH_WRITE_MASK_ALL,DepthFunc:func,FrontFace:stencil,BackFace:stencil,..Default::default() },Some(out)).map_err(err)?;
        }
        let mut crack_normal=None; let mut crack_reversed=None; let mut crack_blend=None;
        for (func,out) in [(D3D11_COMPARISON_LESS_EQUAL,&mut crack_normal),(D3D11_COMPARISON_GREATER_EQUAL,&mut crack_reversed)] {
            device.CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC { DepthEnable:true.into(),DepthWriteMask:D3D11_DEPTH_WRITE_MASK_ZERO,DepthFunc:func,FrontFace:stencil,BackFace:stencil,..Default::default() },Some(out)).map_err(err)?;
        }
        let mut blend=D3D11_BLEND_DESC::default();
        blend.RenderTarget[0]=D3D11_RENDER_TARGET_BLEND_DESC { BlendEnable:true.into(),SrcBlend:D3D11_BLEND_SRC_ALPHA,DestBlend:D3D11_BLEND_INV_SRC_ALPHA,BlendOp:D3D11_BLEND_OP_ADD,SrcBlendAlpha:D3D11_BLEND_ZERO,DestBlendAlpha:D3D11_BLEND_ONE,BlendOpAlpha:D3D11_BLEND_OP_ADD,RenderTargetWriteMask:D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8 };
        device.CreateBlendState(&blend,Some(&mut crack_blend)).map_err(err)?;
        let mut probe_vs=None; let mut probe_ps=None;
        device.CreateVertexShader(&compile(s!("probeVs"),s!("vs_5_0"))?,None,Some(&mut probe_vs)).map_err(err)?;
        device.CreatePixelShader(&compile(s!("probePs"),s!("ps_5_0"))?,None,Some(&mut probe_ps)).map_err(err)?;
        let mut probe_tex=None; let mut probe_rtv=None; let mut staging=None;
        let mut desc=D3D11_TEXTURE2D_DESC { Width:17,Height:1,MipLevels:1,ArraySize:1,Format:DXGI_FORMAT_R32_FLOAT,SampleDesc:DXGI_SAMPLE_DESC{Count:1,Quality:0},Usage:D3D11_USAGE_DEFAULT,BindFlags:D3D11_BIND_RENDER_TARGET.0 as u32,..Default::default() };
        device.CreateTexture2D(&desc,None,Some(&mut probe_tex)).map_err(err)?;
        device.CreateRenderTargetView(probe_tex.as_ref().unwrap(),None,Some(&mut probe_rtv)).map_err(err)?;
        desc.Usage=D3D11_USAGE_STAGING; desc.BindFlags=0; desc.CPUAccessFlags=D3D11_CPU_ACCESS_READ.0 as u32;
        device.CreateTexture2D(&desc,None,Some(&mut staging)).map_err(err)?;
        Ok(Self { mesh_box:Mailbox::open(false)?,atlas_box:Mailbox::open(true)?,mesh:None,atlas:None,vertices:None,texture:None,
            vs:vs.unwrap(),ps:ps.unwrap(),layout:layout.unwrap(),constants:constants.unwrap(),raster:raster.unwrap(),sampler:sampler.unwrap(),normal:normal.unwrap(),reversed:reversed.unwrap(),depth:None,size:(0,0),
            crack_blend:crack_blend.unwrap(),crack_normal:crack_normal.unwrap(),crack_reversed:crack_reversed.unwrap(),
            probe_vs:probe_vs.unwrap(),probe_ps:probe_ps.unwrap(),probe_tex:probe_tex.unwrap(),probe_rtv:probe_rtv.unwrap(),staging:staging.unwrap(),pending:false,last_probe:0,convention:None,source:0,last_log:0,drawn:None })
    } }
    fn key(p:&Option<Packet>)->Option<(u64,u64)> { p.as_ref().map(|p|(p.field(8),p.field(24))) }
    fn receive(mailbox:&Mailbox,last:Option<(u64,u64)>)->io::Result<Option<Packet>> {
        match mailbox.receive(last) { Err(e) if e.kind()==io::ErrorKind::TimedOut=>Ok(None),other=>other }
    }
    unsafe fn upload(&mut self,device:&ID3D11Device)->io::Result<()> { unsafe {
        if let Some(p)=Self::receive(&self.atlas_box,Self::key(&self.atlas))? {
            let start=Instant::now(); let mut texture=None; let mut view=None;
            let data=D3D11_SUBRESOURCE_DATA { pSysMem:p.bytes[HEADER..].as_ptr().cast(),SysMemPitch:p.count()*4,..Default::default() };
            device.CreateTexture2D(&D3D11_TEXTURE2D_DESC { Width:p.count(),Height:p.height(),MipLevels:1,ArraySize:1,Format:DXGI_FORMAT_R8G8B8A8_UNORM,SampleDesc:DXGI_SAMPLE_DESC{Count:1,Quality:0},Usage:D3D11_USAGE_IMMUTABLE,BindFlags:D3D11_BIND_SHADER_RESOURCE.0 as u32,..Default::default() },Some(&data),Some(&mut texture)).map_err(err)?;
            device.CreateShaderResourceView(texture.as_ref().unwrap(),None,Some(&mut view)).map_err(err)?;
            write_log(Role::Skyrim,format!("event=SCENE_UPLOAD kind=2 snapshot={} upload_us={}",p.field(24),start.elapsed().as_micros()))?;
            self.texture=view; self.atlas=Some(p);
        }
        if let Some(p)=Self::receive(&self.mesh_box,Self::key(&self.mesh))? {
            let start=Instant::now(); let mut buffer=None;
            if p.count()>0 {
                let data=D3D11_SUBRESOURCE_DATA { pSysMem:p.bytes[HEADER..].as_ptr().cast(),..Default::default() };
                device.CreateBuffer(&D3D11_BUFFER_DESC { ByteWidth:p.count()*24,Usage:D3D11_USAGE_IMMUTABLE,BindFlags:D3D11_BIND_VERTEX_BUFFER.0 as u32,..Default::default() },Some(&data),Some(&mut buffer)).map_err(err)?;
            }
            write_log(Role::Skyrim,format!("event=SCENE_UPLOAD kind=1 snapshot={} upload_us={}",p.field(24),start.elapsed().as_micros()))?;
            self.vertices=buffer; self.mesh=Some(p);
        }
        Ok(())
    } }
    unsafe fn probe(&mut self,c:&ID3D11DeviceContext1,depth:&ID3D11ShaderResourceView)->io::Result<()> { unsafe {
        if self.pending {
            let mut mapped=D3D11_MAPPED_SUBRESOURCE::default();
            match c.Map(&self.staging,0,D3D11_MAP_READ,D3D11_MAP_FLAG_DO_NOT_WAIT.0 as u32,Some(&mut mapped)) {
                Ok(())=> {
                    let mut samples=[0_f32;16]; ptr::copy_nonoverlapping(mapped.pData.cast(),samples.as_mut_ptr(),16);
                    let center=*mapped.pData.cast::<f32>().add(16); c.Unmap(&self.staging,0);
                    self.pending=false;
                    let finite=samples.iter().all(|d|d.is_finite() && (0.0..=1.0).contains(d));
                    samples.sort_by(f32::total_cmp);
                    let usable=finite && samples[15]-samples[0]>1e-7;
                    self.convention=usable.then_some(samples[8]<0.5);
                    write_log(Role::Skyrim,format!("event=SCENE_DEPTH min={} median={} max={} usable={} reversed={} convention=inferred_from_samples center={} sampled_ms={}",samples[0],samples[8],samples[15],usable,samples[8]<0.5,center,self.last_probe))?;
                }
                Err(e) if e.code().0 as u32==0x887a000a=>return Ok(()),
                Err(e)=>return Err(err(e)),
            }
        }
        if now_ms().saturating_sub(self.last_probe)<500 { return Ok(()); }
        self.last_probe=now_ms();
        c.OMSetRenderTargets(Some(&[Some(self.probe_rtv.clone())]),None);
        c.OMSetBlendState(None,None,u32::MAX); c.OMSetDepthStencilState(None,0);
        c.RSSetState(&self.raster); c.RSSetViewports(Some(&[D3D11_VIEWPORT { Width:17.0,Height:1.0,MaxDepth:1.0,..Default::default() }]));
        c.IASetInputLayout(None); c.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        c.VSSetShader(&self.probe_vs,None); c.PSSetShader(&self.probe_ps,None);
        c.PSSetShaderResources(1,Some(&[Some(depth.clone())])); c.Draw(3,0);
        c.OMSetRenderTargets(None,None); c.CopyResource(&self.staging,&self.probe_tex); self.pending=true;
        Ok(())
    } }
    unsafe fn frame(&mut self,device:&ID3D11Device,c:&ID3D11DeviceContext1,target:&ID3D11RenderTargetView,width:u32,height:u32)->io::Result<()> { unsafe {
        self.upload(device)?;
        let api=API.get().unwrap();
        let camera_fn:unsafe extern "C" fn()->usize=std::mem::transmute(api.camera);
        let camera=camera_fn();
        let raw_depth:*mut c_void=read(api.renderer,0x2018+0x88);
        if camera==0 || raw_depth.is_null() { self.wait("missing_camera_or_depth"); self.convention=None; return Ok(()); }
        let depth=ID3D11ShaderResourceView::from_raw_borrowed(&raw_depth).unwrap();
        let mut view_desc=D3D11_SHADER_RESOURCE_VIEW_DESC::default(); depth.GetDesc(&mut view_desc);
        if view_desc.ViewDimension!=D3D_SRV_DIMENSION_TEXTURE2D { self.wait("unsupported_depth_view"); return Ok(()); }
        if self.source!=raw_depth as usize { self.source=raw_depth as usize; self.convention=None; self.last_probe=0; self.pending=false; }
        self.probe(c,depth)?;
        let Some(sky_reversed)=self.convention else { self.wait("depth_not_yet_usable"); return Ok(()); };
        let Some((sample,anchor))=scene::alignment() else { return Ok(()); };
        let (Some(mesh),Some(atlas))=(&self.mesh,&self.atlas) else { self.wait("awaiting_mesh_or_atlas"); return Ok(()); };
        if mesh.field(8)!=sample.sender || mesh.field(16)!=sample.world || atlas.field(8)!=sample.sender || atlas.field(16)!=sample.world || mesh.field(64)!=atlas.field(64) || now_ms().saturating_sub(mesh.field(32))>2000 || mesh.count()==0 { return Ok(()); }
        let near:f32=read(camera,0x160); let far:f32=read(camera,0x164); let ortho:u8=read(camera,0x168);
        let cam:[f32;3]=read(camera,0xA0); let rotation:[[f32;3];3]=read(camera,0x7C); let matrix:[[f32;4];4]=read(camera,0x110);
        let port:[f32;4]=read(camera,0x174);
        if !port.iter().zip([0.0,1.0,1.0,0.0]).all(|(a,b)|a.is_finite() && (*a-b).abs()<0.001) { self.wait("unsupported_camera_viewport"); return Ok(()); }
        if ortho!=0 || !near.is_finite() || !far.is_finite() || near<=0.0 || far<=near || !matrix.iter().flatten().chain(cam.iter()).all(|v|v.is_finite()) { self.wait("invalid_camera_projection"); return Ok(()); }
        let mut constants=[0_f32;28];
        for r in 0..4 { for col in 0..3 { constants[r*4+col]=matrix[r][col]; }
            constants[r*4+3]=(f64::from(matrix[r][3])+(0..3).map(|col|f64::from(matrix[r][col])*f64::from(cam[col])).sum::<f64>()) as f32;
        }
        let clip=|distance:f32| -> [f32;4] { std::array::from_fn(|r|constants[r*4+3]+(0..3).map(|j|constants[r*4+j]*rotation[j][0]*distance).sum::<f32>()) };
        let a=clip(100.0); let b=clip(10000.0); let check=clip(500.0);
        if !check.iter().all(|v|v.is_finite()) || check[3]<=0.0 || (check[0]/check[3]).abs()>0.2 || (check[1]/check[3]).abs()>0.2 || a[3].abs()<0.001 || b[3].abs()<0.001 { self.wait("camera_matrix_check_failed"); return Ok(()); }
        let reversed=a[2]/a[3]>b[2]/b[3];
        // Straight after a door, load or death the newest mesh can still be from the place
        // just left. Skip it and wait for the next one; this must never disable drawing.
        let Some(origin)=scene::mesh_origin(mesh.origin(),sample,anchor) else { self.wait("mesh_from_another_place"); return Ok(()); };
        for j in 0..3 { constants[16+j]=origin[j]-cam[j]; }
        constants[20]=near; constants[21]=far; constants[22]=if sky_reversed { 1.0 } else { 0.0 };
        constants[24]=width as f32; constants[25]=height as f32;
        if self.size!=(width,height) {
            let mut texture=None; let mut view=None;
            device.CreateTexture2D(&D3D11_TEXTURE2D_DESC { Width:width,Height:height,MipLevels:1,ArraySize:1,Format:DXGI_FORMAT_D32_FLOAT,SampleDesc:DXGI_SAMPLE_DESC{Count:1,Quality:0},Usage:D3D11_USAGE_DEFAULT,BindFlags:D3D11_BIND_DEPTH_STENCIL.0 as u32,..Default::default() },None,Some(&mut texture)).map_err(err)?;
            device.CreateDepthStencilView(texture.as_ref().unwrap(),None,Some(&mut view)).map_err(err)?;
            self.depth=view; self.size=(width,height);
        }
        let start=Instant::now();
        c.UpdateSubresource(&self.constants,0,None,constants.as_ptr().cast(),0,0);
        c.ClearDepthStencilView(self.depth.as_ref().unwrap(),D3D11_CLEAR_DEPTH.0 as u32,if reversed {0.0} else {1.0},0);
        c.OMSetRenderTargets(Some(&[Some(target.clone())]),self.depth.as_ref());
        c.OMSetBlendState(None,None,u32::MAX); c.OMSetDepthStencilState(if reversed {&self.reversed} else {&self.normal},0);
        c.RSSetState(&self.raster); c.RSSetViewports(Some(&[D3D11_VIEWPORT { Width:width as f32,Height:height as f32,MaxDepth:1.0,..Default::default() }]));
        c.IASetInputLayout(&self.layout); c.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        c.IASetVertexBuffers(0,1,Some([self.vertices.clone()].as_ptr()),Some(&24),Some(&0));
        c.VSSetShader(&self.vs,None); c.PSSetShader(&self.ps,None);
        c.VSSetConstantBuffers(0,Some(&[Some(self.constants.clone())])); c.PSSetConstantBuffers(0,Some(&[Some(self.constants.clone())]));
        c.PSSetShaderResources(0,Some(&[self.texture.clone(),Some(depth.clone())])); c.PSSetSamplers(0,Some(&[Some(self.sampler.clone())]));
        let solid=mesh.count()-mesh.cracks()-mesh.translucent();
        c.Draw(solid,0);
        if mesh.cracks()>0 {
            constants[23]=1.0;
            c.UpdateSubresource(&self.constants,0,None,constants.as_ptr().cast(),0,0);
            c.OMSetBlendState(&self.crack_blend,None,u32::MAX);
            c.OMSetDepthStencilState(if reversed {&self.crack_reversed} else {&self.crack_normal},0);
            c.Draw(mesh.cracks(),solid+mesh.translucent());
        }
        if mesh.translucent()>0 {
            constants[23]=2.0;
            c.UpdateSubresource(&self.constants,0,None,constants.as_ptr().cast(),0,0);
            c.OMSetBlendState(&self.crack_blend,None,u32::MAX);
            c.OMSetDepthStencilState(if reversed {&self.crack_reversed} else {&self.crack_normal},0);
            c.Draw(mesh.translucent(),solid);
        }
        device.GetDeviceRemovedReason().map_err(err)?;
        let key=Some((mesh.field(8),mesh.field(24)));
        if self.drawn!=key {
            write_log(Role::Skyrim,format!("event=SCENE_FLUID_DRAW snapshot={} fluid_opaque_vertices={} fluid_translucent_vertices={}",mesh.field(24),mesh.fluid_opaque(),mesh.fluid_translucent()))?;
            write_log(Role::Skyrim,format!("event=SCENE_TRANSLUCENT_DRAW snapshot={} translucent_vertices={} quads={}",mesh.field(24),mesh.translucent(),mesh.translucent()/6))?;
            let models=mesh.models();
            write_log(Role::Skyrim,format!("event=SCENE_ITEM_DRAW snapshot={} item_vertices={} translucent_item_vertices={} items={} models={} icons={}",mesh.field(24),mesh.items(),mesh.translucent_items(),mesh.item_count(),models,mesh.item_count()-models))?;
            write_log(Role::Skyrim,format!("event=SCENE_CRACK_DRAW snapshot={} crack_vertices={} solid_vertices={}",mesh.field(24),mesh.cracks(),solid))?;
            // Compare the real projection to the near/far reconstruction used by the
            // occlusion shader. These must agree before assuming it measures distance.
            let expected_depth_500=if sky_reversed { near*(far/500.0-1.0)/(far-near) } else { far*(1.0-near/500.0)/(far-near) };
            write_log(Role::Skyrim,format!("event=SCENE_PROJECTION snapshot={} matrix_depth_100={} matrix_depth_500={} matrix_depth_10000={} expected_depth_500={} camera_x={} camera_y={} camera_z={} mc_x={} mc_y={} mc_z={}",mesh.field(24),a[2]/a[3],check[2]/check[3],b[2]/b[3],expected_depth_500,cam[0],cam[1],cam[2],sample.mc[0],sample.mc[1],sample.mc[2]))?;
            write_log(Role::Skyrim,format!("event=SCENE_DRAW sender={} world={} snapshot={} atlas={} vertices={} sky_reversed={} mesh_reversed={} near={} far={} projection_x={} projection_y={} projection_w={} sky_origin_x={} sky_origin_y={} sky_origin_z={} submit_us={}",sample.sender,sample.world,mesh.field(24),mesh.field(64),mesh.count(),sky_reversed,reversed,near,far,check[0]/check[3],check[1]/check[3],check[3],origin[0],origin[1],origin[2],start.elapsed().as_micros()))?;
            self.drawn=key;
        }
        Ok(())
    } }
    fn wait(&mut self,reason:&str) {
        if now_ms().saturating_sub(self.last_log)>1000 { self.last_log=now_ms(); let _=write_log(Role::Skyrim,format!("event=SCENE_WAIT reason={reason}")); }
    }
}
