use gcm_bot::maimai::*;

fn main() {
    let charts = set_mai_charts().unwrap();
    for (title, chart) in charts {
        let jp_lv = chart.jp_lv;
        let in_lv = chart.intl_lv;
        if let (Some(in_lv), Some(jp_lv)) = (in_lv, jp_lv) {
            if let (Some(in_dx), Some(jp_dx)) = (in_lv.dx, jp_lv.dx) {
                if in_dx.exp == jp_dx.exp && (jp_dx.exp_c.is_none() ^ in_dx.exp_c.is_none()) {
                    println!(
                        "{} DX EXP: jp {:?}, in {:?}",
                        title, jp_dx.exp_c, in_dx.exp_c
                    );
                }
                if in_dx.mas == jp_dx.mas && (jp_dx.mas_c.is_none() ^ in_dx.mas_c.is_none()) {
                    println!(
                        "{} DX MAS: jp {:?}, in {:?}",
                        title, jp_dx.mas_c, in_dx.mas_c
                    );
                }
                if in_dx.extra == jp_dx.extra && (jp_dx.extra_c.is_none() ^ in_dx.extra_c.is_none())
                {
                    println!(
                        "{} DX REM: jp {:?}, in {:?}",
                        title, jp_dx.extra_c, in_dx.extra_c
                    );
                }
            }
            if let (Some(in_st), Some(jp_st)) = (in_lv.st, jp_lv.st) {
                if in_st.exp == jp_st.exp && (jp_st.exp_c.is_none() ^ in_st.exp_c.is_none()) {
                    println!(
                        "{} ST EXP: jp {:?}, in {:?}",
                        title, jp_st.exp_c, in_st.exp_c
                    );
                }
                if in_st.mas == jp_st.mas && (jp_st.mas_c.is_none() ^ in_st.mas_c.is_none()) {
                    println!(
                        "{} ST MAS: jp {:?}, in {:?}",
                        title, jp_st.mas_c, in_st.mas_c
                    );
                }
                if in_st.extra == jp_st.extra && (jp_st.extra_c.is_none() ^ in_st.extra_c.is_none())
                {
                    println!(
                        "{} ST REM: jp {:?}, in {:?}",
                        title, jp_st.extra_c, in_st.extra_c
                    );
                }
            }
        }
    }
}
