//! Kessig Cagebreakers — `{4}{G}` 3/4 Human Rogue. "Whenever this creature
//! attacks, create a 2/2 green Wolf creature token that's tapped and attacking
//! for each creature card in your graveyard."
//!
//! GAP: CreateToken does not support "enters tapped and attacking"; using
//! ForEach with graveyard creature count, creating tokens without the
//! tapped-attacking modifier.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kessig Cagebreakers");
    let _wolf = reg.interner_mut().intern("Wolf");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::graveyard_size(state, trig.controller);
    if n == 0 {
        return Vec::new();
    }
    let wolf = reg.interner().lookup("Wolf")
        .expect("Wolf interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    // GAP: CreateToken does not support "enters tapped and attacking".
    let token = TokenDefinition {
        name: wolf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::ForEach {
        targets: (0..n).map(|_| NULL_OBJECT_ID).collect(),
        effect: Box::new(Effect::CreateToken { controller: trig.controller, token }),
    }]
}
