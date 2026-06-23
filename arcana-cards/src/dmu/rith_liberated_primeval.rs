//! Rith, Liberated Primeval — `{2}{R}{G}{W}` 5/5 Legendary Creature — Dragon.
//!
//! Oracle:
//! * Flying, ward {2}.
//! * Other Dragons you control have ward {2}. (static grant — GAP)
//! * At the beginning of your end step, if a creature or planeswalker an
//!   opponent controlled was dealt excess damage this turn, create a 4/4 red
//!   Dragon creature token with flying.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rith, Liberated Primeval");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: static "Other Dragons you control have ward {2}" — keyword-granting
    // static, not a triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: intervening-if "if a creature or planeswalker an opponent
            // controlled was dealt excess damage this turn" — no excess-damage
            // condition predicate exists; gate omitted (fires unconditionally).
            intervening_if: None,
            effect: make_dragon_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_dragon_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
