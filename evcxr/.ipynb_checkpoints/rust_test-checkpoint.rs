{
 "cells": [
  {
   "cell_type": "code",
   "execution_count": 3,
   "id": "1109abf7-e1cd-4f17-b2c5-05b789cd9890",
   "metadata": {},
   "outputs": [
    {
     "name": "stdout",
     "output_type": "stream",
     "text": [
      "Hello\n"
     ]
    }
   ],
   "source": [
    "println!(\"Hello\");"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 4,
   "id": "26ef880a-5d26-4f34-9f21-29bd3596cf88",
   "metadata": {},
   "outputs": [],
   "source": [
    "fn stat(list: &mut [i32]) {\n",
    "println!(\"\\taverage: {}\",\n",
    "        list.iter().sum::<i32>() as f32 / list.len() as f32\n",
    "    );\n",
    "    println!(\n",
    "        \"\\tmedian: {:?}\",\n",
    "        match {\n",
    "            list.sort();\n",
    "            list.get(list.len() / 2)\n",
    "        } {\n",
    "            None => panic!(\"found none\"),\n",
    "            Some(i) => i,\n",
    "        }\n",
    "    );\n",
    "    use std::collections::HashMap;\n",
    "    let mut map = HashMap::new();\n",
    "    for i in list.iter() {\n",
    "        // let count = map.entry(i).or_insert(0);\n",
    "        // *count += 1;\n",
    "        *map.entry(i).or_insert(0) += 1;\n",
    "    }\n",
    "\n",
    "    let map = map.into_iter();\n",
    "    // println!(\n",
    "    //     \"mode: {:?}\",\n",
    "    //     match map.max_by_key(|&(_val, count)| count) {\n",
    "    //         Some(i) => i.0,\n",
    "    //         None => panic!(\"None\"),\n",
    "    //     }\n",
    "    // );\n",
    "    println!(\n",
    "        \"\\tmode: {:?}\",\n",
    "        map.max_by_key(|&(_val, count)| count)\n",
    "            .map(|(val, _count)| val)\n",
    "            .expect(\"Nothing\")\n",
    "    );\n",
    "}\n",
    "\n",
    "fn piglatin(pstr: &str) {\n",
    "    match &pstr[0..1] {\n",
    "        \"a\" | \"e\" | \"i\" | \"o\" | \"u\" => println!(\"\\t{}-hay\", &pstr[0..]),\n",
    "        _ => println!(\"\\t{}-{}ay\", &pstr[1..], &pstr[0..1]),\n",
    "    }\n",
    "}\n"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 10,
   "id": "efd2904c-1425-4cc9-b10f-82d1b073f524",
   "metadata": {},
   "outputs": [
    {
     "name": "stdout",
     "output_type": "stream",
     "text": [
      "Average, Median and Mode -> \n",
      "\taverage: 4.090909\n",
      "\tmedian: 4\n",
      "\tmode: 0\n",
      "Pig Latin -> \n",
      "\tirst-fay\n",
      "\tapple-hay\n",
      "\tig-pay\n",
      "\tatin-lay\n"
     ]
    }
   ],
   "source": [
    "let mut list = vec![0, 0, 2, 5, 8, 7, 1, 4, 3, 6, 9];\n",
    "println!(\"Average, Median and Mode -> \");\n",
    "stat(&mut list);\n",
    "let mut plist = vec![\"first\", \"apple\", \"pig\", \"latin\"];\n",
    "println!(\"Pig Latin -> \");\n",
    "for string in &mut plist {\n",
    "    piglatin(string);\n",
    "}\n",
    ";"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 11,
   "id": "85eef4b4-3e51-4517-8067-ffb319e4b03a",
   "metadata": {},
   "outputs": [
    {
     "data": {
      "image/png": "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"
     },
     "execution_count": 11,
     "metadata": {},
     "output_type": "execute_result"
    }
   ],
   "source": [
    ":dep image = \"0.23\"\n",
    ":dep evcxr_image = \"1.1\"\n",
    "\n",
    "use evcxr_image::ImageDisplay;\n",
    "\n",
    "image::ImageBuffer::from_fn(256, 256, |x, y| {\n",
    "    if (x as i32 - y as i32).abs() < 3 {\n",
    "        image::Rgb([0, 0, 255])\n",
    "    } else {\n",
    "        image::Rgb([0, 0, 0])\n",
    "    }\n",
    "})"
   ]
  },
  {
   "cell_type": "code",
   "execution_count": 12,
   "id": "2e86ae7f-f2ed-4396-8761-5507e7fc2ece",
   "metadata": {},
   "outputs": [
    {
     "data": {
      "text/html": [
       "<table>\n",
       "<tr><td>:clear</td><td>Clear all state, keeping compilation cache</td></tr>\n",
       "<tr><td>:dep</td><td>Add dependency. e.g. :dep regex = \"1.0\"</td></tr>\n",
       "<tr><td>:efmt</td><td>Set the formatter for errors returned by ?</td></tr>\n",
       "<tr><td>:explain</td><td>Print explanation of last error</td></tr>\n",
       "<tr><td>:fmt</td><td>Set output formatter (default: {:?})</td></tr>\n",
       "<tr><td>:help</td><td>Print command help</td></tr>\n",
       "<tr><td>:internal_debug</td><td>Toggle various internal debugging code</td></tr>\n",
       "<tr><td>:last_compile_dir</td><td>Print the directory in which we last compiled</td></tr>\n",
       "<tr><td>:last_error_json</td><td>Print the last compilation error as JSON (for debugging)</td></tr>\n",
       "<tr><td>:linker</td><td>Set/print linker. Supported: system, lld</td></tr>\n",
       "<tr><td>:load_config</td><td>Reloads startup configuration files</td></tr>\n",
       "<tr><td>:offline</td><td>Set offline mode when invoking cargo</td></tr>\n",
       "<tr><td>:opt</td><td>Set optimization level (0/1/2)</td></tr>\n",
       "<tr><td>:preserve_vars_on_panic</td><td>Try to keep vars on panic (0/1)</td></tr>\n",
       "<tr><td>:quit</td><td>Quit evaluation and exit</td></tr>\n",
       "<tr><td>:sccache</td><td>Set whether to use sccache (0/1).</td></tr>\n",
       "<tr><td>:time_passes</td><td>Toggle printing of rustc pass times (requires nightly)</td></tr>\n",
       "<tr><td>:timing</td><td>Toggle printing of how long evaluations take</td></tr>\n",
       "<tr><td>:toolchain</td><td>Set which toolchain to use (e.g. nightly)</td></tr>\n",
       "<tr><td>:vars</td><td>List bound variables and their types</td></tr>\n",
       "<tr><td>:version</td><td>Print Evcxr version</td></tr>\n",
       "</table>\n"
      ],
      "text/plain": [
       ":clear            Clear all state, keeping compilation cache\n",
       ":dep              Add dependency. e.g. :dep regex = \"1.0\"\n",
       ":efmt             Set the formatter for errors returned by ?\n",
       ":explain          Print explanation of last error\n",
       ":fmt              Set output formatter (default: {:?})\n",
       ":help             Print command help\n",
       ":internal_debug   Toggle various internal debugging code\n",
       ":last_compile_dir Print the directory in which we last compiled\n",
       ":last_error_json  Print the last compilation error as JSON (for debugging)\n",
       ":linker           Set/print linker. Supported: system, lld\n",
       ":load_config      Reloads startup configuration files\n",
       ":offline          Set offline mode when invoking cargo\n",
       ":opt              Set optimization level (0/1/2)\n",
       ":preserve_vars_on_panic Try to keep vars on panic (0/1)\n",
       ":quit             Quit evaluation and exit\n",
       ":sccache          Set whether to use sccache (0/1).\n",
       ":time_passes      Toggle printing of rustc pass times (requires nightly)\n",
       ":timing           Toggle printing of how long evaluations take\n",
       ":toolchain        Set which toolchain to use (e.g. nightly)\n",
       ":vars             List bound variables and their types\n",
       ":version          Print Evcxr version\n"
      ]
     },
     "execution_count": 12,
     "metadata": {},
     "output_type": "execute_result"
    }
   ],
   "source": [
    ":help"
   ]
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Rust",
   "language": "rust",
   "name": "rust"
  },
  "language_info": {
   "codemirror_mode": "rust",
   "file_extension": ".rs",
   "mimetype": "text/rust",
   "name": "Rust",
   "pygment_lexer": "rust",
   "version": ""
  }
 },
 "nbformat": 4,
 "nbformat_minor": 5
}
