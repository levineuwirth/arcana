//! Desculpting Blast — `{1}{U}` instant. "Return target nonland
//! permanent to its owner's hand. If it was attacking, create a 1/1
//! colorless Drone artifact creature token with flying and 'This token
//! can block only creatures with flying.'"
//! Attacking status checked directly against `state.combat` at resolve.
//! GAP: the token's "can block only creatures with flying" restriction
//! is not expressible; the token is a plain 1/1 flying Drone.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Desculpting Blast");
    let _drone = reg.interner_mut().intern("Drone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return target nonland permanent to its owner's hand. If it was attacking, create a 1/1 colorless Drone artifact creature token with flying.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into()),
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
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut effects = vec![Effect::ReturnToHand { target: *id }];
    // "If it was attacking" — checked at resolution against combat state.
    if state.combat.as_ref().is_some_and(|c| c.is_attacker(*id)) {
        let drone = reg.interner().lookup("Drone").expect("Drone interned during register()");
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(drone);
        effects.push(Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: drone,
                colors: ColorSet::default(),
                types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        });
    }
    effects
}
