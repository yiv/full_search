#import "FullSearchPlugin.h"
#if __has_include(<full_search/full_search-Swift.h>)
#import <full_search/full_search-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "full_search-Swift.h"
#endif

@implementation FullSearchPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftFullSearchPlugin registerWithRegistrar:registrar];
}
@end
