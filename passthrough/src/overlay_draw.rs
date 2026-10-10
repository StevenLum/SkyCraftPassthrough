//! World drawing before Skyrim HUD; Minecraft hands/HUD at Present. Isolated D3D state.
use std::{cell::RefCell, ffi::c_void, io, ptr, sync::atomic::{AtomicUsize,Ordering}, time::Instant};
use ::windows::{core::{Interface,PCSTR,s}, Win32::Graphics::{Direct3D::*,Direct3D::Fxc::D3DCompile,Direct3D11::*,Dxgi::*,Dxgi::Common::*}};
use crate::{Role,native::write_log,overlay::{self,Channel,Frame},windows::{SharedMemory,now_ms}};

static ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static SWAP:AtomicUsize=AtomicUsize::new(0);
static HUD_ORIGINAL:AtomicUsize=AtomicUsize::new(0);
type Present=unsafe extern "system" fn(*mut c_void,u32,u32)->i32;
#[link(name="kernel32")]
unsafe extern "system" { fn VirtualProtect(address:*mut c_void,size:usize,protection:u32,old:*mut u32)->i32; }

pub unsafe fn install(window_global:usize)->io::Result<()> {
    if ORIGINAL.load(Ordering::Acquire)!=0 { return Ok(()); }
    let window=unsafe { ptr::read(window_global as *const usize) };
    if window==0 { return Err(io::Error::other("Skyrim render window missing")); }
    let swap=unsafe { ptr::read((window+0x18) as *const usize) };
    if swap==0 { return Err(io::Error::other("Skyrim swap chain missing")); }
    let table=unsafe { ptr::read(swap as *const usize) };
    let slot=(table+8*8) as *mut usize;
    let mut old=0;
    if unsafe { VirtualProtect(slot.cast(),8,0x04,&mut old) }==0 { return Err(io::Error::last_os_error()); }
    SWAP.store(swap,Ordering::Release);
    ORIGINAL.store(unsafe { ptr::read(slot) },Ordering::Release);
    unsafe { ptr::write(slot,present as *const () as usize); }
    let mut unused=0;
    if unsafe { VirtualProtect(slot.cast(),8,old,&mut unused) }==0 {
        let _=write_log(Role::Skyrim,"event=OVERLAY_WARNING message=Present_hook_page_protection_restore_failed");
    }
    write_log(Role::Skyrim,"event=OVERLAY_HOOK installed=1")
}

// HUDMenu inherits IMenu::PostDisplay, which calls uiMovie->Display (CommonLib).
// Chain the installed function so Skyrim and existing hooks still draw their UI.
pub unsafe fn install_hud(vtable:usize)->io::Result<()> {
    if HUD_ORIGINAL.load(Ordering::Acquire)!=0 { return Ok(()); }
    let slot=(vtable+6*8) as *mut usize;
    let original=unsafe { ptr::read(slot) };
    if original==0 { return Err(io::Error::other("HUD display function missing")); }
    let mut old=0;
    if unsafe { VirtualProtect(slot.cast(),8,0x04,&mut old) }==0 { return Err(io::Error::last_os_error()); }
    HUD_ORIGINAL.store(original,Ordering::Release);
    unsafe { ptr::write(slot,hud_display as *const () as usize); }
    let mut unused=0;
    if unsafe { VirtualProtect(slot.cast(),8,old,&mut unused) }==0 {
        let _=write_log(Role::Skyrim,"event=SCENE_STAGE_WARNING message=HUD_hook_page_protection_restore_failed");
    }
    write_log(Role::Skyrim,"event=SCENE_STAGE_HOOK stage=before_skyrim_hud vtable_id=215362 slot=6 installed=1")
}

unsafe extern "C" fn hud_display(menu:*mut c_void) {
    let raw=SWAP.load(Ordering::Acquire) as *mut c_void;
    if !raw.is_null() {
        let result=std::panic::catch_unwind(|| STATE.with(|cell|->io::Result<()> {
            let mut state=cell.borrow_mut();
            if state.failed || state.world_failed || overlay::allowed().is_none() { return Ok(()); }
            // Present initializes the shared renderer. Start world drawing next frame.
            let Some(renderer)=state.renderer.as_ref() else { return Ok(()); };
            let swap=unsafe { IDXGISwapChain::from_raw_borrowed(&raw) }.unwrap();
            unsafe { renderer.draw_world(swap) }.map_err(io::Error::other)?;
            state.world_seen=true;
            if now_ms().saturating_sub(state.world_log)>1000 {
                state.world_log=now_ms();
                write_log(Role::Skyrim,"event=SCENE_STAGE stage=before_skyrim_hud present_world_draw=0")?;
            }
            Ok(())
        }));
        if !matches!(result,Ok(Ok(()))) {
            let message=match result { Ok(Err(e))=>e.to_string(),_=>"pre-HUD rendering panic".into() };
            let _=write_log(Role::Skyrim,format!("event=SCENE_STAGE_DISABLED message={message}"));
            STATE.with(|cell|cell.borrow_mut().world_failed=true);
        }
    }
    let original:unsafe extern "C" fn(*mut c_void)=unsafe { std::mem::transmute(HUD_ORIGINAL.load(Ordering::Acquire)) };
    unsafe { original(menu); }
}

#[derive(Default)]
struct State { renderer:Option<Renderer>, channel:Option<Channel>, memory:Option<SharedMemory>, failed:bool,
    world_failed:bool,world_seen:bool,world_log:u64,world_wait_log:u64 }
thread_local! { static STATE:RefCell<State>=RefCell::new(State::default()); }
impl State {
    fn ready(&self,ready:bool) {
        if let Some(memory)=&self.memory { let _=memory.update(|r| {
            if r.slots[0].pid==std::process::id() {
                r.render_session=r.slots[0].session; r.render_ms=now_ms(); r.render_ready=u32::from(ready);
            }
            Ok(())
        }); }
    }
    unsafe fn frame(&mut self,swap:&IDXGISwapChain)->io::Result<()> {
        if self.failed { self.ready(false); return Ok(()); }
        if self.memory.is_none() { self.memory=Some(SharedMemory::open(&overlay::name())?); }
        if overlay::allowed().is_none() { self.ready(false); return Ok(()); }
        if self.renderer.is_none() {
            self.renderer=Some(unsafe { Renderer::new(swap) }.map_err(io::Error::other)?);
            self.channel=Some(Channel::open()?);
            write_log(Role::Skyrim,"event=OVERLAY_READY format=RGBA8 blend=premultiplied state_restore=context_swap")?;
        }
        let renderer=self.renderer.as_mut().unwrap();
        if !self.world_seen && now_ms().saturating_sub(self.world_wait_log)>1000 {
            self.world_wait_log=now_ms();
            write_log(Role::Skyrim,"event=SCENE_STAGE_WAIT reason=no_pre_hud_pass_on_present_thread present_world_draw=0")?;
        }
        self.world_seen=false;
        let incoming=match self.channel.as_ref().unwrap().receive() {
            Err(e) if e.kind()==io::ErrorKind::TimedOut=>None,
            result=>result?,
        };
        if let Some(frame)=incoming {
            let key=(frame.sender,frame.world,frame.number);
            if renderer.last.as_ref().map(|f|(f.sender,f.world,f.number))!=Some(key) {
                let start=Instant::now();
                if overlay::digest(&frame.pixels)!=(frame.checksum,frame.alpha) {
                    return Err(io::Error::other("overlay checksum mismatch"));
                }
                unsafe { renderer.upload(&frame) }.map_err(io::Error::other)?;
                write_log(Role::Skyrim,format!("event=OVERLAY_RECV sender={} world={} frame={} width={} height={} bytes={} checksum={} alpha_pixels={} validate_upload_us={} age_ms={}",
                    frame.sender,frame.world,frame.number,frame.width,frame.height,frame.pixels.len(),frame.checksum,frame.alpha,start.elapsed().as_micros(),now_ms().saturating_sub(frame.sent)))?;
                renderer.last=Some(frame);
            }
        }
        if let (Some(permit),Some(frame))=(overlay::allowed(),renderer.last.as_ref()) {
            if permit.sender==frame.sender && permit.world==frame.world && now_ms().saturating_sub(frame.sent)<=500 {
                let start=Instant::now();
                unsafe { renderer.draw(swap) }.map_err(io::Error::other)?;
                if renderer.drawn!=Some((frame.sender,frame.world,frame.number)) {
                    write_log(Role::Skyrim,format!("event=OVERLAY_DRAW sender={} world={} frame={} submit_us={} width={} height={}",frame.sender,frame.world,frame.number,start.elapsed().as_micros(),frame.width,frame.height))?;
                    renderer.drawn=Some((frame.sender,frame.world,frame.number));
                }
            }
        }
        self.ready(true);
        Ok(())
    }
}

unsafe extern "system" fn present(raw:*mut c_void,interval:u32,flags:u32)->i32 {
    if raw as usize==SWAP.load(Ordering::Acquire) && flags&1==0 {
        let result=std::panic::catch_unwind(|| STATE.with(|state| {
            // Borrow the game's COM reference; never release its ownership.
            let swap=unsafe { IDXGISwapChain::from_raw_borrowed(&raw) }.unwrap();
            unsafe { state.borrow_mut().frame(swap) }
        }));
        if !matches!(result,Ok(Ok(()))) {
            let message=match result { Ok(Err(e))=>e.to_string(),_=>"overlay panic".into() };
            let _=write_log(Role::Skyrim,format!("event=OVERLAY_DISABLED message={message}"));
            STATE.with(|state| { let mut state=state.borrow_mut(); state.failed=true; state.ready(false); state.renderer=None; });
        }
    }
    let original:Present=unsafe { std::mem::transmute(ORIGINAL.load(Ordering::Acquire)) };
    unsafe { original(raw,interval,flags) }
}

// Drop restores every graphics stage even if an upload/draw returns an error.
struct Restore<'a> { context:&'a ID3D11DeviceContext1, old:Option<ID3DDeviceContextState> }
impl Drop for Restore<'_> { fn drop(&mut self) { unsafe { self.context.SwapDeviceContextState(self.old.as_ref(),None); } } }
struct Renderer {
    device:ID3D11Device, context:ID3D11DeviceContext1, isolated:ID3DDeviceContextState,
    vertex:ID3D11VertexShader,pixel:ID3D11PixelShader,blend:ID3D11BlendState,
    depth:ID3D11DepthStencilState,raster:ID3D11RasterizerState,sampler:ID3D11SamplerState,
    texture:Option<ID3D11Texture2D>,view:Option<ID3D11ShaderResourceView>,size:(u32,u32),
    last:Option<Frame>,drawn:Option<(u64,u64,u64)>,
}
const SHADER:&[u8]=br#"
Texture2D image : register(t0); SamplerState filtering : register(s0);
struct Vertex { float4 position : SV_Position; float2 uv : TEXCOORD0; };
Vertex vs(uint id : SV_VertexID) {
    Vertex v; float2 uv=float2((id << 1) & 2,id & 2);
    v.position=float4(uv.x*2-1,1-uv.y*2,0,1);
    v.uv=float2(uv.x,1-uv.y); return v;
}
float4 ps(Vertex v) : SV_Target { return image.Sample(filtering,v.uv); }
"#;
unsafe fn compile(entry:PCSTR,target:PCSTR)->::windows::core::Result<Vec<u8>> {
    let mut code=None; let mut errors=None;
    let result=unsafe { D3DCompile(SHADER.as_ptr().cast(),SHADER.len(),s!("PassthroughOverlay"),None,None,entry,target,0,0,&mut code,Some(&mut errors)) };
    if let Some(errors)=errors {
        let bytes=unsafe { std::slice::from_raw_parts(errors.GetBufferPointer().cast::<u8>(),errors.GetBufferSize()) };
        let _=write_log(Role::Skyrim,format!("event=OVERLAY_SHADER message={}",String::from_utf8_lossy(bytes).replace(['\r','\n']," ")));
    }
    result?;
    let code=code.unwrap();
    Ok(unsafe { std::slice::from_raw_parts(code.GetBufferPointer().cast::<u8>(),code.GetBufferSize()) }.to_vec())
}
impl Renderer {
    unsafe fn draw_world(&self,swap:&IDXGISwapChain)->::windows::core::Result<()> { unsafe {
        let back:ID3D11Texture2D=swap.GetBuffer(0)?;
        let mut desc=D3D11_TEXTURE2D_DESC::default(); back.GetDesc(&mut desc);
        if desc.SampleDesc.Count!=1 { return Err(::windows::core::Error::from_hresult(::windows::core::HRESULT(0x80004001_u32 as i32))); }
        let mut target=None; self.device.CreateRenderTargetView(&back,None,Some(&mut target))?;
        let mut old=None; self.context.SwapDeviceContextState(&self.isolated,Some(&mut old));
        let _restore=Restore{context:&self.context,old};
        crate::scene_draw::draw(&self.device,&self.context,target.as_ref().unwrap(),desc.Width,desc.Height);
        self.context.OMSetRenderTargets(None,None);
        self.device.GetDeviceRemovedReason()?;
        Ok(())
    } }
    unsafe fn new(swap:&IDXGISwapChain)->::windows::core::Result<Self> { unsafe {
        let device:ID3D11Device=swap.GetDevice()?;
        let device1:ID3D11Device1=device.cast()?;
        let context=device1.GetImmediateContext1()?;
        let mut isolated=None;
        device1.CreateDeviceContextState(device.GetCreationFlags() & 1,&[device.GetFeatureLevel()],D3D11_SDK_VERSION,&ID3D11Device::IID,None,Some(&mut isolated))?;
        let mut vertex=None; let mut pixel=None;
        device.CreateVertexShader(&compile(s!("vs"),s!("vs_5_0"))?,None,Some(&mut vertex))?;
        device.CreatePixelShader(&compile(s!("ps"),s!("ps_5_0"))?,None,Some(&mut pixel))?;
        let mut blend=None; let mut blend_desc=D3D11_BLEND_DESC::default();
        blend_desc.RenderTarget[0]=D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable:true.into(),SrcBlend:D3D11_BLEND_ONE,DestBlend:D3D11_BLEND_INV_SRC_ALPHA,BlendOp:D3D11_BLEND_OP_ADD,
            SrcBlendAlpha:D3D11_BLEND_ONE,DestBlendAlpha:D3D11_BLEND_INV_SRC_ALPHA,BlendOpAlpha:D3D11_BLEND_OP_ADD,
            RenderTargetWriteMask:D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8 };
        device.CreateBlendState(&blend_desc,Some(&mut blend))?;
        let mut depth=None;
        device.CreateDepthStencilState(&D3D11_DEPTH_STENCIL_DESC { DepthEnable:false.into(),DepthWriteMask:D3D11_DEPTH_WRITE_MASK_ZERO,
            DepthFunc:D3D11_COMPARISON_ALWAYS,StencilEnable:false.into(),FrontFace:D3D11_DEPTH_STENCILOP_DESC {
                StencilFailOp:D3D11_STENCIL_OP_KEEP,StencilDepthFailOp:D3D11_STENCIL_OP_KEEP,StencilPassOp:D3D11_STENCIL_OP_KEEP,StencilFunc:D3D11_COMPARISON_ALWAYS },
            BackFace:D3D11_DEPTH_STENCILOP_DESC { StencilFailOp:D3D11_STENCIL_OP_KEEP,StencilDepthFailOp:D3D11_STENCIL_OP_KEEP,StencilPassOp:D3D11_STENCIL_OP_KEEP,StencilFunc:D3D11_COMPARISON_ALWAYS },..Default::default() },Some(&mut depth))?;
        let mut raster=None;
        device.CreateRasterizerState(&D3D11_RASTERIZER_DESC { FillMode:D3D11_FILL_SOLID,CullMode:D3D11_CULL_NONE,DepthClipEnable:true.into(),..Default::default() },Some(&mut raster))?;
        let mut sampler=None;
        device.CreateSamplerState(&D3D11_SAMPLER_DESC { Filter:D3D11_FILTER_MIN_MAG_MIP_LINEAR,AddressU:D3D11_TEXTURE_ADDRESS_CLAMP,AddressV:D3D11_TEXTURE_ADDRESS_CLAMP,
            AddressW:D3D11_TEXTURE_ADDRESS_CLAMP,ComparisonFunc:D3D11_COMPARISON_NEVER,MaxAnisotropy:1,MaxLOD:f32::MAX,..Default::default() },Some(&mut sampler))?;
        Ok(Self{device,context,isolated:isolated.unwrap(),vertex:vertex.unwrap(),pixel:pixel.unwrap(),blend:blend.unwrap(),depth:depth.unwrap(),raster:raster.unwrap(),sampler:sampler.unwrap(),texture:None,view:None,size:(0,0),last:None,drawn:None})
    } }
    unsafe fn upload(&mut self,frame:&Frame)->::windows::core::Result<()> { unsafe {
        if self.size!=(frame.width,frame.height) {
            let mut texture=None;
            self.device.CreateTexture2D(&D3D11_TEXTURE2D_DESC { Width:frame.width,Height:frame.height,MipLevels:1,ArraySize:1,Format:DXGI_FORMAT_R8G8B8A8_UNORM,
                SampleDesc:DXGI_SAMPLE_DESC{Count:1,Quality:0},Usage:D3D11_USAGE_DEFAULT,BindFlags:D3D11_BIND_SHADER_RESOURCE.0 as u32,..Default::default() },None,Some(&mut texture))?;
            let mut view=None; self.device.CreateShaderResourceView(texture.as_ref().unwrap(),None,Some(&mut view))?;
            self.texture=texture; self.view=view; self.size=(frame.width,frame.height);
        }
        self.context.UpdateSubresource(self.texture.as_ref().unwrap(),0,None,frame.pixels.as_ptr().cast(),frame.width*4,0);
        self.device.GetDeviceRemovedReason()?;
        Ok(())
    } }
    unsafe fn draw(&self,swap:&IDXGISwapChain)->::windows::core::Result<()> { unsafe {
        // Do not retain back-buffer references across Present/ResizeBuffers.
        let back:ID3D11Texture2D=swap.GetBuffer(0)?;
        let mut desc=D3D11_TEXTURE2D_DESC::default(); back.GetDesc(&mut desc);
        let mut target=None; self.device.CreateRenderTargetView(&back,None,Some(&mut target))?;
        let mut old=None; self.context.SwapDeviceContextState(&self.isolated,Some(&mut old));
        let _restore=Restore{context:&self.context,old};
        self.context.OMSetRenderTargets(Some(&[target]),None);
        self.context.OMSetBlendState(&self.blend,None,u32::MAX);
        self.context.OMSetDepthStencilState(&self.depth,0);
        self.context.RSSetState(&self.raster);
        self.context.RSSetViewports(Some(&[D3D11_VIEWPORT { Width:desc.Width as f32,Height:desc.Height as f32,MinDepth:0.0,MaxDepth:1.0,..Default::default() }]));
        self.context.IASetInputLayout(None);
        self.context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        self.context.VSSetShader(&self.vertex,None); self.context.PSSetShader(&self.pixel,None);
        self.context.PSSetShaderResources(0,Some(&[self.view.clone()]));
        self.context.PSSetSamplers(0,Some(&[Some(self.sampler.clone())]));
        self.context.Draw(3,0);
        // Our reusable state must not retain the back buffer either.
        self.context.OMSetRenderTargets(None,None);
        self.context.PSSetShaderResources(0,Some(&[None]));
        self.device.GetDeviceRemovedReason()?;
        Ok(())
    } }
}
