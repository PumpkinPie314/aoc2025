import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.stream.Collectors;
import java.util.stream.Stream;

class Main {
    public static void main(String[] args) throws IOException{
        ArrayList<Integer> turns = Files.lines(Paths.get("test"))
            .map(line -> Integer.parseInt(line.replace("L", "-").replace("R", "")))
            .collect(Collectors.toCollection(ArrayList::new));
        // System.out.println(turns);
        int pos = 50;
        int zero_count = 0;
        for (int turn : turns) {
            int turn_dir = 0;
            if (turn == 0) continue;
            if (turn < 0) turn_dir = -1 ; 
            if (turn > 0) turn_dir = 1 ;
            for (int i = 0; i < Math.abs(turn); i ++) {
                pos += turn_dir;
                if (pos == -1) pos = 99;
                if (pos == 100) pos = 0;
                if (pos == 0) zero_count++;
            }
            
            System.out.println("" + turn + "\t" + pos + "\t" + zero_count + ((pos == 0) ? " pointing at 0!": "") );
        }
    }
}