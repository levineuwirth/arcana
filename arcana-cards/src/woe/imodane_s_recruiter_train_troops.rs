//! Imodane's Recruiter // Train Troops — `{2}{R}` // `{4}{W}` red Adventure creature.
//! Creature: 2/2 Human Knight. When this creature enters, creatures you control get +1/+0 and haste until end of turn.
//! Adventure (Train Troops — Sorcery): Create two 2/2 white Knight creature tokens with vigilance.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imodane's Recruiter");
    let adv_name = reg.interner_mut().intern("Train Troops");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create two 2/2 white Knight creature tokens with vigilance.".into(),
        target_requirements: vec![],
        modal: None,
        effect: train_troops_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pump_haste,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn etb_pump_haste(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    ids.into_iter()
        .flat_map(|id| {
            vec![
                Effect::Pump { target: id, power: 1, toughness: 0, duration: Duration::EndOfTurn, keywords: vec![] },
                Effect::GrantKeyword { target: id, keyword: KeywordAbility::Haste, duration: Duration::EndOfTurn },
            ]
        })
        .collect()
}

fn train_troops_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight").expect("Knight interned");
    let mut ts = SubtypeSet::default();
    ts.0.insert(knight);
    let token = TokenDefinition {
        name: knight,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: ts,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
