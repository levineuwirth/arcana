//! Lulu, Inspiring Hollyphant — `{2}{R}{W}` 2/4 Legendary Elephant Angel.
//! Flying.
//! "Whenever you attack with one or more other creatures with flying, create
//!  that many 1/1 white Soldier creature tokens that are tapped and attacking."
//!
//! Flying is wired. The attack trigger has no exact "whenever you attack with
//! one or more …" batch trigger condition, so it is modeled as SelfAttacks
//! (Lulu attacking), counting attacking flying creatures you control at
//! resolution and minting that many tapped+attacking Soldier tokens. Minor
//! fidelity GAP: Lulu herself is counted among "other" attacking flyers (no
//! self-exclusion in the combat filter).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lulu, Inspiring Hollyphant");
    let elephant = reg.interner_mut().intern("Elephant");
    let angel = reg.interner_mut().intern("Angel");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: make_soldiers,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_soldiers(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &arcana_core::targets::ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_keyword(KeywordAbility::Flying)
            .attacking_only(),
        trig.controller,
    );
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..n)
        .map(|_| Effect::CreateTokenTappedAttacking {
            controller: trig.controller,
            token: token.clone(),
        })
        .collect()
}
