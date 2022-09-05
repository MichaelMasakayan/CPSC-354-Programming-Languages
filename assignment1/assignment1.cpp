#include <iostream>
using namespace std;
int gcd(int n, int m) {
   if (m == 0)
   return n;
   if(n>m)
   {
      gcd(m,n-m);
   }
   else if(n<m)
   {
      gcd(n,m-n);
   }
   return gcd(m, n % m);
}
int main() {
    int n,m;
    cout<<"type the number for the first argument of the GCD"<<'\n';
    cin>>n;
    cout<<"type the number for the first argument of the GCD"<<'\n';
    cin>>m;
   cout<<"GCD of "<< n <<" and "<< m <<" is "<< gcd(n, m);
   return 0;
}