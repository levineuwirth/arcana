//! Aang and Katara — `{3}{G}{W}{U}` 5/5 legendary green/white/blue Human
//! Avatar Ally. "Whenever Aang and Katara enter or attack, create X 1/1
//! white Ally creature tokens, where X is the number of tapped artifacts
//! and/or creatures you control."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
    let name = reg.interner_mut().intern("Aang and Katara");
    let human = reg.interner_mut().intern("Human");
    let avatar = reg.interner_mut().intern("Avatar");
    let ally = reg.interner_mut().intern("Ally");
    let _ally2 = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(avatar);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: create_ally_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_ally_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_ally_tokens(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let tapped_filter = ObjectFilter::new()
        .controlled_by(ControllerConstraint::You)
        .tapped_only();
    let n = script::count_matching(state, &tapped_filter, trig.controller);
    let ally = reg.interner().lookup("Ally")
        .expect("Ally interned during register()");
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(ally);
        let token = TokenDefinition {
            name: ally,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken { controller: trig.controller, token });
    }
    effects
}
