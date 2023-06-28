O java foi um caso mais complicado. Para podermos usar a função do Rust, temos de implementar métodos específicos à JNI, que estão na lib.rs no nosso projeto (libzermia).

Depois temos de fazer uma Java interface:
RustInterface.java:
```java
public class RustInterface {

	static {
		System.loadLibrary("zermia_lib"); // A lib.so no nosso target/release
	}
	
	public static class Message {
		public int code;
		public int ipSrc;
		public int ipDest;
		public byte[] msg = new byte[32];
		public boolean returnStatus;
	}
	
		public native boolean sendMessage(int code, int ipSrc, int ipDest, byte[] message, boolean returnStatus);
}
```

Criamos o .h a partir desta interface:

```bash
javac -h . RustInterface.java
```


No nosso programa Java:

```java
RustInterface rustInterface = new RustInterface();

boolean return_value = rustInterface.sendMessage(21, 21, 21, "hello world JAVADA".getBytes(), false);

if (return_value) {

System.out.println("Recebemos true!");

} else {

System.out.println("Recebemos false!");

}
```

Compilamos o programa normalmente.

Sempre que corrermos o programa Java, fazemos da seguinte forma:
```bash
java -Djava.library.path=/home/hugocardante/JHShadow/build/src/target/release myJavaProgram
```

Isto garante que passamos a library à JVM. 