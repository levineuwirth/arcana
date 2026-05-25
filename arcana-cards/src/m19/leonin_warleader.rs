//! Leonin Warleader — `{2}{W}{W}` 4/4 white Cat Soldier. "Whenever this
//! creature attacks, create two 1/1 white Cat creature tokens with lifelink
//! that are tapped and attacking."
//!
//! Note: the tokens being "tapped and attacking" when created is a
//! replacement-effect detail the engine handles via the CreateToken +
//! attack context; the lifelink keyword is encoded on the token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leonin Warleader");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);

    // Pre-intern token subtype
    let _cat_token = reg.interner_mut().intern("Cat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: create_cat_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_cat_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat")
        .expect("Cat interned during register()");
    let make_token = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(cat);
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: cat,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Lifelink],
                abilities: vec![],
            },
        }
    };
    vec![make_token(), make_token()]
}
