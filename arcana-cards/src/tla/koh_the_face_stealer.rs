//! Koh, the Face Stealer — `{4}{B}{B}` 6/6 Legendary Shapeshifter Spirit.
//!
//! * "When Koh enters, exile up to one other target creature."
//! * "Whenever another nontoken creature dies, you may exile it." — wired as a
//!   dies trigger that exiles the dying card from the graveyard (the "you may"
//!   optionality is omitted — see GAP).
//! * "Pay 1 life: Choose a creature card exiled with Koh." + "Koh has all
//!   activated and triggered abilities of the last chosen card." — the
//!   choose-from-exile + ability-grafting machinery is not expressible, GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koh, the Face Stealer");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // "another nontoken creature dies"
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: exile_dying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "Pay 1 life: Choose a creature card exiled with Koh" + Koh gains all
    // activated/triggered abilities of the last chosen card — ability grafting and
    // choose-from-source-exile are not expressible.
}

fn etb_exile(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ExilePermanent { target: *id }]
}

fn exile_dying(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "you may" optionality is omitted; exiles the dying card unconditionally.
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ExileFromGraveyard { target: id }]
}
