public class RustInterface {
  static {
      System.loadLibrary("zermia_lib"); //Especificar onde está a minha lib
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