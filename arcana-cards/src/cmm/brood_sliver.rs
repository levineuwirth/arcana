//! Brood Sliver — `{4}{G}` 3/3 green creature. "Whenever a Sliver deals combat
//! damage to a player, its controller may create a 1/1 colorless Sliver creature
//! token."
//!
//! GAP: trigger — "a Sliver" (any Sliver, not just this one) maps to
//! DamageDealt with a Sliver source_filter; subtype filter for source not
//! directly expressible in DamageDealt.source_filter. Using creature filter.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brood Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — source_filter for "Sliver subtype" not expressible; using creature filter
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: sliver_damage_create_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sliver_damage_create_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let sliver = reg.interner().lookup("Sliver")
        .expect("Sliver interned during register()");
    let mut st = SubtypeSet::default();
    st.0.insert(sliver);
    let token = TokenDefinition {
        name: sliver,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
