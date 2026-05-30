//! Ojer Taq, Deepest Foundation // Temple of Civilization
//!
//! Front: `{4}{W}{W}` Legendary Creature — God 6/6
//! Vigilance.
//! If one or more creature tokens would be created under your control, three times
//! that many of those tokens are created instead.
//! (GAP: token-tripling replacement effect not modeled — no replacement Effect variant.)
//! When Ojer Taq dies, return it to the battlefield tapped and transformed under its
//! owner's control.
//! (GAP: "return tapped" not expressible — ReturnFromGraveyardToBattlefield places untapped.)
//!
//! Back: Temple of Civilization — Legendary Land (no mana cost, transforms from front).
//! {T}: Add {W}.
//! {2}{W}, {T}: Transform this land. Activate only if you attacked with three or more
//! creatures this turn and only as a sorcery.
//! (GAP: back-face mana ability {T}: Add {W} not modeled — back-face activated abilities
//!  not auto-installed on transform.)
//! (GAP: back-face activated transform ability with "attacked with 3+ creatures" condition
//!  not modeled.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ojer Taq, Deepest Foundation");
    let god_sub = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Temple of Civilization");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "When Ojer Taq dies, return it to the battlefield tapped and transformed."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Return to battlefield (untapped — GAP: "tapped" not expressible),
    // then transform to back face (Temple of Civilization).
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
