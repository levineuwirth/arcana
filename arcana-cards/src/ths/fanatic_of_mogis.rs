//! Fanatic of Mogis — `{3}{R}` 4/2 Minotaur Shaman.
//! "When this creature enters, it deals damage to each opponent equal
//! to your devotion to red."
//!
//! GAP: script helpers do not include a devotion counter; using
//! count_matching on red permanents you control as a proxy.
//! GAP: devotion counts {R} pips in mana costs, not just the count of
//! red permanents — approximation only; full devotion support is an
//! engine gap.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fanatic of Mogis");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_deal_devotion_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_deal_devotion_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: devotion (pip-counting) not available in script API;
    // approximating with count of red permanents you control.
    let devotion = script::count_matching(
        state,
        &ObjectFilter::permanent().with_colors(ColorSet::red()),
        trig.controller,
    );
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: devotion,
            source: trig.source,
        })
        .collect()
}
