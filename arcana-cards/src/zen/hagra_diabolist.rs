//! Hagra Diabolist — `{4}{B}` 3/2 black Creature — Ogre Shaman Ally.
//! "Whenever this creature or another Ally you control enters, you may have
//! target player lose life equal to the number of Allies you control."
//! Note: "this creature or another Ally" — using ZoneChange creature entering
//! under your control as trigger approximation (can't filter to Ally subtype
//! at trigger build time; subtype_filter requires reg at resolve time).
//! Dynamic amount — count Allies you control via script.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetRequirement, TargetChoice};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hagra Diabolist");
    let ogre = reg.interner_mut().intern("Ogre");
    let shaman = reg.interner_mut().intern("Shaman");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(shaman);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "this creature or another Ally" filter — can't filter
                // by Ally subtype at trigger build time; using generic creature
                // entering your battlefield
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: on_ally_enters_lose_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn on_ally_enters_lose_life(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let ally_filter = script::subtype_filter(reg, "Ally").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &ally_filter, trig.controller);
    vec![Effect::LoseLife { player: *p, amount: n }]
}
