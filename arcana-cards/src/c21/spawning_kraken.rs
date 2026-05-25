//! Spawning Kraken — `{5}{U}` 6/6 blue Kraken. "Whenever a Kraken, Leviathan,
//! Octopus, or Serpent you control deals combat damage to a player, create a 9/9
//! blue Kraken creature token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawning Kraken");
    let kraken_sub = reg.interner_mut().intern("Kraken");
    let _leviathan = reg.interner_mut().intern("Leviathan");
    let _octopus = reg.interner_mut().intern("Octopus");
    let _serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    // The trigger condition is: "Whenever a Kraken, Leviathan, Octopus, or Serpent
    // you control deals combat damage to a player". This uses DamageDealt with a
    // source_filter for those subtypes. We use with_subtypes_any for the OR of four
    // subtypes.
    let kraken_id = reg.interner_mut().intern("Kraken");
    let leviathan_id = reg.interner_mut().intern("Leviathan");
    let octopus_id = reg.interner_mut().intern("Octopus");
    let serpent_id = reg.interner_mut().intern("Serpent");
    let source_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![kraken_id, leviathan_id, octopus_id, serpent_id]);
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: create_kraken_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_kraken_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kraken = reg.interner().lookup("Kraken")
        .expect("Kraken interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);
    let token = TokenDefinition {
        name: kraken,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
