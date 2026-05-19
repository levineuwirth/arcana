//! Emergency Eject — `{2}{W}` instant, "Destroy target nonland permanent. Its controller
//! creates a 1/1 colorless Lander artifact creature token."
//!
//! GAP: Lander token (the token's controller is the destroyed permanent's controller, not
//! the spell controller; identifying the destroyed permanent's controller at resolve time).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emergency Eject");
    let _lander = reg.interner_mut().intern("Lander");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy target nonland permanent. Its controller creates a 1/1 colorless Lander artifact creature token.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().without_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let lander = reg.interner().lookup("Lander").expect("Lander interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lander);
    let token = TokenDefinition {
        name: lander,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token should be created under the destroyed permanent's controller, not spell controller
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
