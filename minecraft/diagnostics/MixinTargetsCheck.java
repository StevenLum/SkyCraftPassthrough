import java.lang.classfile.ClassFile;
import java.lang.classfile.ClassModel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.zip.ZipFile;

/**
 * Checks every hook this mod places into Minecraft against Minecraft's own class files, without
 * starting the game. The build does not do this: a misspelt or renamed target compiles and
 * packages cleanly, and then stops Minecraft while it is loading.
 *
 * Usage: MixinTargetsCheck &lt;mixin source folder&gt; &lt;passthrough.mixins.json&gt; &lt;Minecraft jar&gt;
 * (run by tools\test-java-rules.ps1 before every build). Checked: that the hooked class
 * exists; every @Inject method name (with its descriptor when one is written); every @At
 * target call; every @Shadow field or method; every @Accessor and @Invoker name; and that the
 * list in passthrough.mixins.json matches the files.
 */
public final class MixinTargetsCheck {
    private static final List<String> problems=new ArrayList<>();
    private static int checked;
    private record Members(Set<String> methods,Set<String> methodsWithType,Set<String> fields) {}
    private static final Map<String,Members> cache=new HashMap<>();
    private static ZipFile jar;

    /** Members declared by one class of the jar, or null when the jar has no such class. */
    private static Members members(String internalName) throws Exception {
        if(cache.containsKey(internalName)) return cache.get(internalName);
        Members found=null;
        var entry=jar.getEntry(internalName+".class");
        if(entry!=null) {
            ClassModel model;
            try(var in=jar.getInputStream(entry)) {model=ClassFile.of().parse(in.readAllBytes());}
            var methods=new HashSet<String>();var typed=new HashSet<String>();var fields=new HashSet<String>();
            for(var method:model.methods()) {
                methods.add(method.methodName().stringValue());
                typed.add(method.methodName().stringValue()+method.methodType().stringValue());
            }
            for(var field:model.fields()) fields.add(field.fieldName().stringValue());
            found=new Members(methods,typed,fields);
        }
        cache.put(internalName,found);
        return found;
    }
    private static void check(boolean ok,String file,String what) {
        checked++;
        if(!ok) problems.add(file+": "+what);
    }
    /** "name" or "name(descriptor)return" against one class's own methods. */
    private static boolean hasMethod(Members members,String reference) {
        return reference.contains("(") ? members.methodsWithType().contains(reference) : members.methods().contains(reference);
    }

    public static void main(String[] args) throws Exception {
        Path folder=Path.of(args[0]),config=Path.of(args[1]);
        jar=new ZipFile(args[2]);
        var files=new ArrayList<Path>();
        try(var stream=Files.list(folder)) {stream.filter(p->p.toString().endsWith(".java")).sorted().forEach(files::add);}
        var names=new HashSet<String>();
        for(Path path:files) {
            String file=path.getFileName().toString(),source=Files.readString(path);
            names.add(file.substring(0,file.length()-5));
            Matcher mixin=Pattern.compile("@Mixin\\(\\s*([\\w.]+)\\.class\\s*\\)").matcher(source);
            if(!mixin.find()) {check(false,file,"no @Mixin(Some.class) line found");continue;}
            // The hooked class is named by its simple name; its import gives the full name.
            String simple=mixin.group(1),outer=simple.contains(".") ? simple.substring(0,simple.indexOf('.')) : simple;
            Matcher imported=Pattern.compile("import\\s+([\\w.]+\\."+Pattern.quote(outer)+")\\s*;").matcher(source);
            if(!imported.find()) {check(false,file,"no import found for hooked class "+simple);continue;}
            String internal=imported.group(1).replace('.','/')+simple.substring(outer.length()).replace('.','$');
            Members target=members(internal);
            check(target!=null,file,"hooked class "+internal+" is not in the Minecraft jar");
            if(target==null) continue;

            Matcher method=Pattern.compile("@Inject\\s*\\(\\s*method\\s*=\\s*\"([^\"]+)\"").matcher(source);
            while(method.find()) check(hasMethod(target,method.group(1)),file,internal+" has no method "+method.group(1));

            // A call the hook is attached to, written as Lowner;name(descriptor)return.
            Matcher call=Pattern.compile("target\\s*=\\s*\"L([^;\"]+);([^\"(]+)(\\([^\"]*)\"").matcher(source);
            while(call.find()) {
                Members owner=members(call.group(1));
                if(owner==null) {check(false,file,"class "+call.group(1)+" named in an @At target is not in the Minecraft jar");continue;}
                check(owner.methodsWithType().contains(call.group(2)+call.group(3)),file,call.group(1)+" has no method "+call.group(2)+call.group(3));
            }

            // "@Shadow ... name;" is a field of the hooked class, "@Shadow ... name(" a method.
            Matcher shadow=Pattern.compile("@Shadow\\b[^;{(]*?\\b(\\w+)\\s*([;(])").matcher(source);
            while(shadow.find()) {
                boolean isMethod=shadow.group(2).equals("(");
                check(isMethod ? target.methods().contains(shadow.group(1)) : target.fields().contains(shadow.group(1)),
                    file,internal+" has no "+(isMethod ? "method " : "field ")+shadow.group(1));
            }
            Matcher accessor=Pattern.compile("@Accessor\\(\\s*\"([^\"]+)\"\\s*\\)").matcher(source);
            while(accessor.find()) check(target.fields().contains(accessor.group(1)),file,internal+" has no field "+accessor.group(1));
            Matcher invoker=Pattern.compile("@Invoker\\(\\s*\"([^\"]+)\"\\s*\\)").matcher(source);
            while(invoker.find()) check(target.methods().contains(invoker.group(1)),file,internal+" has no method "+invoker.group(1));
        }

        // The list Minecraft's loader reads must name exactly the files that exist.
        var listed=new HashSet<String>();
        Matcher entry=Pattern.compile("\"(\\w+(?:Mixin|Access))\"").matcher(Files.readString(config));
        while(entry.find()) check(listed.add(entry.group(1)),config.getFileName().toString(),entry.group(1)+" is listed twice");
        for(String name:listed) check(names.contains(name),config.getFileName().toString(),"lists "+name+" but there is no such file");
        for(String name:names) check(listed.contains(name),config.getFileName().toString(),"does not list "+name+", so that hook would never be applied");

        if(!problems.isEmpty()) {
            for(String problem:problems) System.out.println("FAIL "+problem);
            System.out.println("FAILED: "+problems.size()+" of "+checked+" hook checks. Minecraft would stop while loading.");
            System.exit(1);
        }
        System.out.println("PASS: "+checked+" hook checks against the Minecraft jar ("+files.size()+" hook files).");
    }
}
