//! Colossus of the Blood Age — `{4}{R}{W}` 6/6 Artifact Creature — Construct.
//!
//! "When this creature enters, it deals 3 damage to each opponent and you
//!   gain 3 life." — fully modeled (per-opponent damage + life gain).
//! "When this creature dies, discard any number of cards, then draw that
//!   many cards plus one." — the post-discard draw count depends on how many
//!   cards the controller discards at resolution, which cannot be threaded
//!   into a literal-amount draw, so the dies trigger is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Colossus of the Blood Age");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
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
                effect: etb_burn_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_burn_and_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        out.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(opp),
            amount: 3,
        });
    }
    out.push(Effect::GainLife { player: trig.controller, amount: 3 });
    out
}

fn dies_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard any number of cards, then draw that many cards plus one" — the draw
    // count depends on the number of cards chosen for the any-number discard at
    // resolution, which cannot be threaded into a literal DrawCards amount.
    Vec::new()
}
