//! Syr Konrad's Squire — `{1}{B}` 2/2 black Creature — Human Knight.
//! "Whenever Syr Konrad's Squire dies, or is put into a graveyard from
//! anywhere other than the battlefield, or leaves your graveyard, or is put
//! into exile, or is put into your library, or is returned to your hand from
//! the battlefield or graveyard, or is phased out, it deals 1 damage to each
//! opponent."
//! GAP: trigger — only SelfDies (battlefield→graveyard) can be modeled; the
//! other trigger clauses (non-battlefield graveyard entry, leaves graveyard,
//! exile, library, return to hand, phase out) have no catalog variants.
//! SelfDies used for the primary clause.

use arcana_core::events::DamageTarget;
use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Syr Konrad's Squire");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — only SelfDies modeled; many other trigger
                // clauses (non-battlefield GY entry, leaves GY, exile, library,
                // return to hand, phase out) have no catalog variants.
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies_damage_opponents,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies_damage_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 1,
            source: trig.source,
        })
        .collect()
}
