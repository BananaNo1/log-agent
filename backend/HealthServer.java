import com.sun.net.httpserver.HttpServer;

import java.io.IOException;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.charset.StandardCharsets;

public class HealthServer {
    public static void main(String[] args) throws IOException{
         var address = new InetSocketAddress(
                    InetAddress.getByName("127.0.0.1"),
                    18765
                  );
        HttpServer server = HttpServer.create(address,0);

        server.createContext("/health",exchange->{
            try(exchange){
              if(!"GET".equals(exchange.getRequestMethod())
                || !"/health".equals(exchange.getRequestURI().getPath())){
                  exchange.sendResponseHeaders(404,-1);
                  return;
              }

              byte[] body = "Java backend is running".getBytes(StandardCharsets.UTF_8);

              exchange.getResponseHeaders().set(
                "Content-Type",
                "text/plain;charset=utf-8"
              );

              exchange.sendResponseHeaders(200,body.length);
              exchange.getResponseBody().write(body);
              
            }
        });

      server.start();

      System.out.println(
        "java backend http://127.0.0.1:18765/health"
      );
    }
}
