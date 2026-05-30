//! Purging Stormbrood // Absorb Essence — `{4}{B}` Dragon 4/4 with Adventure
//! face "Absorb Essence" (`{1}{W}` Instant — Omen).
//!
//! Creature face:
//! - Flying
//! - Ward — Pay 2 life. GAP: non-mana ward cost not expressible; omitted.
//! - When this creature enters, remove all counters from up to one target
//!   creature. GAP: "remove all counters" requires enumerating all counter
//!   kinds present on the target at resolution, which is not accessible via
//!   the permitted script helpers. Returning Vec::new().
//!
//! Adventure face "Absorb Essence" (`{1}{W}` Instant):
//! - Target creature gets +2/+2 and gains lifelink and hexproof until end
//!   of turn.
//! - (Then shuffle this card into its owner's library.) — Omen mechanic;
//!   GAP: not modeled in the engine.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Purging Stormbrood");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // Ward — Pay 2 life: non-mana ward cost not expressible; omitted.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // Adventure face "Absorb Essence"
    let adv_name = reg.interner_mut().intern("Absorb Essence");
    let omen_sub = reg.interner_mut().intern("Omen");
    let mut adv_subtypes = SubtypeSet::default();
    adv_subtypes.0.insert(omen_sub);
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        subtypes: adv_subtypes,
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target creature gets +2/+2 and gains lifelink and hexproof until end of turn.".into(),
        target_requirements: vec![TargetRequirement::target_creature()],
        modal: None,
        effect: absorb_essence_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    // ETB trigger: remove all counters from up to one target creature.
    // TargetCount::UpTo(1) to allow "up to one".
    let etb_req = TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_remove_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![etb_req],
            }),
    )
}

fn etb_remove_counters(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "remove all counters" requires enumerating all counter kinds
    // present on the target at resolution, which is not accessible via
    // the permitted script helpers (no script::counters_on(state, id)).
    Vec::new()
}

fn absorb_essence_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // +2/+2 with lifelink and hexproof until end of turn.
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Lifelink, KeywordAbility::Hexproof],
    }]
}
