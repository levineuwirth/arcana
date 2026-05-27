//! Dripping-Tongue Zubera — `{1}{G}` 1/2 green Zubera Spirit.
//! "When this creature dies, create a 1/1 colorless Spirit creature token for each
//! Zubera that died this turn."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dripping-Tongue Zubera");
    let zubera = reg.interner_mut().intern("Zubera");
    let spirit = reg.interner_mut().intern("Spirit");
    let _spirit_token = spirit;
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zubera);
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_spirit_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_spirit_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // N = number of Zubera that died this turn (includes this one)
    let n = script::creatures_of_subtype_died_this_turn(state, reg, "Zubera");
    if n == 0 {
        return Vec::new();
    }
    let spirit = reg.interner().lookup("Spirit").expect("Spirit interned during register()");
    let tokens: Vec<Effect> = (0..n).map(|_| {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(spirit);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: spirit,
                colors: ColorSet::colorless(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }
    }).collect();
    vec![Effect::Sequence(tokens)]
}
