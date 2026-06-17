//! The Unagi of Kyoshi Island — `{3}{U}{U}` 5/5 Legendary Serpent.
//! Flash; Ward—Waterbend {4}; whenever an opponent draws their second
//! card each turn, you draw two cards.
//!
//! Ward—Waterbend {4} is modeled as the parametrized mana Ward {4} (the
//! "tap artifacts/creatures to help pay" rider is engine-side cost
//! payment). The opponent-draws-their-second-card trigger fires on each
//! opponent draw — the "second card each turn" gating is not expressible
//! with the available intervening-if predicates (GAP'd).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Unagi of Kyoshi Island");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Ward(ManaCost::parse("{4}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "their SECOND card each turn" gating is not expressible;
                // fires on each opponent draw instead.
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: draw_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_two(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 2 }]
}
