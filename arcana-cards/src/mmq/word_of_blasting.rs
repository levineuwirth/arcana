//! Word of Blasting — `{1}{R}` instant.
//! "Destroy target Wall. It can't be regenerated. Word of Blasting deals damage equal to
//! that Wall's mana value to the Wall's controller."
//! GAP: targeting creatures of a specific subtype (Wall) via TargetFilter
//! (subtype filtering in TargetFilter not available — only ObjectFilter::creature() shown);
//! GAP: computing target permanent's mana value at resolution (script helpers don't expose mana value);
//! GAP: "can't be regenerated" flag on DestroyPermanent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Word of Blasting");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target Wall. It can't be regenerated. Word of Blasting deals damage equal to that Wall's mana value to the Wall's controller.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: subtype-filtered targeting (Wall subtype not expressible in TargetFilter)
    // GAP: mana value of the targeted permanent not available via script helpers
    // GAP: "can't be regenerated" modifier on DestroyPermanent
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
