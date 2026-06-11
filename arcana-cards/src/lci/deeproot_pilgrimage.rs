//! Deeproot Pilgrimage — `{1}{U}` enchantment (Lost Caverns of Ixalan).
//! "Whenever one or more nontoken Merfolk you control become tapped,
//! create a 1/1 blue Merfolk creature token with hexproof."
//!
//! GAP: the trigger condition — "one or more [filtered permanents you
//! control] become tapped" — has no `TriggerCondition` variant (only
//! `SelfBecomesTapped` exists, which watches THIS enchantment). The closest
//! variant is wired so the card registers; it will not fire for Merfolk.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deeproot Pilgrimage");
    let _merfolk = reg.interner_mut().intern("Merfolk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever one or more nontoken Merfolk you
                // control become tapped" has no TriggerCondition variant;
                // SelfBecomesTapped (this enchantment) is the closest tap
                // condition and will not fire for the Merfolk.
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: make_merfolk_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 blue Merfolk creature token with hexproof."
fn make_merfolk_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merfolk = reg.interner().lookup("Merfolk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: merfolk,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Hexproof],
            abilities: vec![],
        },
    }]
}
