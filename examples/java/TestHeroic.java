import heroic.v1.Query;
import heroic.v1.TimeUnit;

public class TestHeroic {
    public static void main(String[] argv) {
        final Query a = new Query.Builder().query("average by host").build();
        final Query b = new Query.Builder().query("average by host").build();
        final Query c = new Query.Builder().query("average by other").build();

        System.out.println("query = " + a.toString());

        System.out.println("equals b? = " + a.equals(b));
        System.out.println("equals c? = " + a.equals(c));
        System.out.println("equals null? = " + a.equals(null));

        System.out.println("SECONDS = " + TimeUnit.SECONDS);
        System.out.println("SECONDS(number) = " + TimeUnit.SECONDS.getNumber());
        System.out.println("SECONDS(fromValue(1000)) = " + TimeUnit.SECONDS.fromValue(1000));
        System.out.println("SECONDS(fromValue(60000)) = " + TimeUnit.SECONDS.fromValue(60000));
    }
}
