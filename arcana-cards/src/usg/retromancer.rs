//! Retromancer — `{2}{R}{R}` 3/3 red creature. "Whenever this creature
//! becomes the target of a spell or ability, this creature deals 3 damage
//! to that spell or ability's controller."
//!
//! GAP: trigger — no TriggerCondition for "whenever this creature becomes
//! the target of a spell or ability". Using SelfEntersBattlefield as
//! closest available; verify pipeline will flag.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Retromancer");
    let lizard = reg.interner_mut().intern("Lizard");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — no TriggerCondition for "becomes target of spell or ability"
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: targeted_deal_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn targeted_deal_3(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Deal 3 to the controller of the targeting spell/ability.
    // With the GAP trigger, no target info is available; dealing to first opponent.
    let opponents = script::opponents(state, trig.controller);
    if let Some(&opp) = opponents.first() {
        vec![Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 3,
            source: trig.source,
        }]
    } else {
        Vec::new()
    }
}
