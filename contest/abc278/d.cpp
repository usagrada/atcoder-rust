#include <bits/stdc++.h>
using namespace std;

int main()
{
    int n;
    cin >> n;
    long long standard = 0;
    vector<long long> a(n);
    set<int> s;
    for (int i = 0; i < n; i++)
    {
        cin >> a[i];
        s.insert(i+1);
    }
    int q;
    cin >> q;
    string ans = "";
    for (int i = 0; i < q; i++)
    {
        int type;
        cin >> type;
        if (type == 1)
        {
            long long x;
            cin >> x;
            standard = x;
            s.clear();
        }
        else if (type == 2)
        {
            int i;
            long long x;
            cin >> i >> x;
            if (s.find(i) == s.end())
            {
                s.insert(i);
                a[i - 1] = x;
            }
            else
            {
                a[i - 1] += x;
            }
        }
        else if (type == 3)
        {
            int i;
            cin >> i;
            if (s.find(i) == s.end())
            {
                ans += to_string(standard) + "\n";
            }
            else
            {
                ans += to_string(a[i - 1] + standard) + "\n";
            }
        }
    }
    cout << ans << flush;
}
