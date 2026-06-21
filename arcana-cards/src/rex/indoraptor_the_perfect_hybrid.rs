//! Indoraptor, the Perfect Hybrid — `{1}{B/G}{R}` 3/1 Legendary Dinosaur Mutant.
//! Bloodthirst X (enters with X +1/+1 counters, X = damage to your opponents
//! this turn) — GAP: Bloodthirst is a fixed-N keyword, not a variable X.
//! Menace.
//! Enrage — Whenever Indoraptor is dealt damage, choose an opponent at random.
//! Indoraptor deals damage equal to its power to that player unless they
//! sacrifice a nontoken creature of their choice.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Indoraptor, the Perfect Hybrid");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/G}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Bloodthirst X" — the Bloodthirst keyword carries a fixed u8 N,
        // not a runtime-variable X (= damage dealt to opponents this turn), so
        // it is omitted rather than emit a wrong fixed value.
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfIsDealtDamage { combat_only: false },
            intervening_if: None,
            effect: enrage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enrage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the damage is gated by "unless they sacrifice a nontoken creature" —
    // OptionalPayment supports only Mana/Life costs, not a sacrifice cost, so
    // the conditional damage cannot be expressed. Emitting unconditional damage
    // would be materially wrong, so the whole Enrage effect is omitted.
    Vec::new()
}
