public class myJavaProgram{

  public static void main(String args[]){
   
    RustInterface rustInterface = new RustInterface();
    boolean return_value = rustInterface.sendMessage(21, 21, 21, "hello world JAVADA".getBytes(), false);

    if (return_value) {
        System.out.println("Recebemos true!");
    } else {
        System.out.println("Recebemos false!");
    }
  }
}
