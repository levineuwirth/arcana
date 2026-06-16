//! Bruna, Light of Alabaster — `{3}{W}{W}{U}` 5/5 Legendary Angel.
//! Flying, vigilance. When she attacks or blocks, attach any number of
//! Auras (on the battlefield, in your graveyard, and/or hand) to her.
//!
//! GAP: the attach-any-number-of-Auras-from-multiple-zones effect is not
//! expressible with the allowed primitives. The trigger is wired so the
//! event fires, but its resolution is a no-op (best effort).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bruna, Light of Alabaster");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: attach_auras,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attach_auras(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: attach any number of Auras to Bruna from the battlefield, and put
    // onto the battlefield attached to her any number of Aura cards that could
    // enchant her from your graveyard and/or hand — multi-zone mass-attach is
    // not expressible. (Also note: SelfBlocksOrBecomesBlocked does not cover
    // the "attacks" half of "attacks or blocks".)
    Vec::new()
}
