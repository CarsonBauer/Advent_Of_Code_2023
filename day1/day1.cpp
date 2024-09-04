#include <ctype.h>
#include <fstream>
#include <iostream>
#include <string>

int read_file() {
  std::ifstream myfile;
  myfile.open("input.txt");
  std::string mystring;
  int total = 0;

  if (myfile.is_open()) {
    while (myfile) {
      std::string myline;
      std::string myDigits;
      std::getline(myfile, myline);

      if (myline.size() > 0) {
        for (int i = 0; i < myline.size(); i++) {
          if (isdigit(myline[i])) {
            myDigits += myline[i];
          }
        }

        std::string firstLastChars = "";
        firstLastChars += myDigits[0];
        firstLastChars += myDigits[myDigits.size() - 1];
        total += std::stoi(firstLastChars);
      }
    }
  }

  return total;
}

int main() {
  int total = read_file();
  std::cout << "Total: " << total << "\n";
}
