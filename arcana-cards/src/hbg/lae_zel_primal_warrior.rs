//! Lae'zel, Primal Warrior — `{3}{G}{W}` 3/6 Legendary Creature — Gith Warrior.
//!
//! Double strike.
//! When this creature enters or specializes, other creatures you control and
//! creature cards in your hand perpetually get +1/+1.
//!
//! # Decomposition
//! * Keyword line — `Double strike`.
//! * "When this creature enters … +1/+1" → ETB trigger (id 1).
//! * "… or specializes …" → specialize trigger (id 2).
//!
//! Best-effort: the buff is modeled as a permanent white/green Anthem
//! (creatures you control get +1/+1). True fidelity is out of reach with the
//! demonstrated API:
//! * GAP: "perpetually" is not modeled (no perpetual-effect primitive); the
//!   Anthem is a continuous static instead, which dims if the source leaves.
//! * GAP: the buff also pumps "creature cards in your hand" — there is no
//!   hand-zone continuous buff primitive.
//! * Minor: Anthem buffs ALL your creatures including Lae'zel, where the
//!   oracle says "other"; the demonstrated API has no self-exclusion knob.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lae'zel, Primal Warrior");
    let gith = reg.interner_mut().intern("Gith");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gith);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: buff_your_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: buff_your_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn buff_your_creatures(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually" + "creature cards in your hand" + "other" not modeled;
    // best-effort permanent Anthem over your battlefield creatures.
    vec![Effect::Anthem {
        controller: trig.controller,
        power: 1,
        toughness: 1,
        duration: Duration::Permanent,
    }]
}
