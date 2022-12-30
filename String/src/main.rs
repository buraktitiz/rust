#[allow(unused_variables)]
fn main() {
    let example_str: &str = "Howdy"; // & değişmezliğini belirtiyor.
    let example_string: String = String::from("Partner"); // Büyük harfle başlayan String değiştirilebilir

    let string_from_str: String = example_str.to_string(); // burada formu değiştirdik ve değiştirilemez olan stringi (to_string kullanarak) değştirilebilir yaptık

    //let string_from_str2:String="Bazı kodlanmış dizeler"; // Sabit kodlu dizeler de kullanılabilir ancak onu atamaya çalıştığımız zaman bir hata üretecek.
    // Tek yapmamız gereken sonuna .to_string() eklemek

    let string_from_str2: String = "Bazı kodlanmış dizeler".to_string(); // Şimdi çalışacaktır.

    let string_from_hardcoded = String::from("Bazıları kodlanmış"); // Bir dize dilimine dayalı yeni bir dize oluşturduk.
    let string_from_hardcoded2 = String::from(example_str);

    // string from yöntemini ya string sabit bilgisinde ya da string dilim değişkendinde kullanabiliriz.
    // burada dikkat edilmesi gereken şudur: Nasıl başlattığımıza bağlı olarak bir string dilimi değil, bir string türü istediğimizi otomatik olarak belirleyebilir.
    // derleyici için ne tür bir değişken olması gerektiği açıksai önceki değişkenlerimizde yaptığımız gibi açıkça tanımlamamız gerekmiyordu, derleyici bunu genellikle kendisi için çözebilir. Bir dize diliminden bir diziye gidiyor.

    // şimdi tam tersi şeklinde gidelim ve bunun için dizinin önünde & işaretini kullanalım.
    let str_from_string: &str = &example_string; // bunun gibi bir dizeden bir dize dilimi oluşturduğumuzda alsında karakterlerin bir kopyasını oluşturmuyor ve onları bir değişken bir bellek yuvasına itmiyor. Sadece dizenin orijinal belleğine işaret ediyor.

    // şimdi dizeleri nasıl birleştireceğimiz hakkında çalışmalar yapalım.
    // örneğin iki sabit kodlu dize değişmezini bir araya getirelim ve ne olduğunu görelim.
    //let test = "first" + "second"; // bu şekilde bir kullanımda derleme hatası alacağız.
    // Onları bir diziye koyabilir ve ardından concat'ı çağırabiliriz.
    let combine_string_literals = ["first", "second"].concat();

    // let string_plus_str=example_string+example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Bazı kodlanmış veriler");
    mut_string.push('m');

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string; // bir dizeyi diğer dizeye çevirirken yapmamız gereken şey sadece ilk dizede & olmaması.

    let str_from_substring: &str = &example_str[0..2];
}
