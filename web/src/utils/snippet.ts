import testpy from '../corpus/test.py?raw';

export const getRawSourceCode = (key: string) : string => {
    switch (key) {
        case "test.py":
            return testpy;
        default:
            return "no source code available!"
    }
}