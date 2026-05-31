//! Kaslem's Stonetree // Kaslem's Strider
//!
//! Front face (Artifact): When this artifact enters, look at the top six cards
//! of your library. You may put a land card from among them onto the battlefield
//! tapped. Put the rest on the bottom in a random order.
//! Craft with Cave {5}{G} — Craft is not an expressible mechanic; see GAP.
//! Back face (Artifact Creature — Golem), reached via Craft (transform).

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, CardFace};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaslem's Stonetree");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    // Back face: Artifact Creature — Golem.
    let back_name = reg.interner_mut().intern("Kaslem's Strider");
    let golem = reg.interner_mut().intern("Golem");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(golem);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_dig_land,
                trigger_zones: Vec::new(),
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: Craft with Cave {5}{G} (exile this + exile a Cave: return transformed)
    // is not an expressible activated-ability/cost shape; the back face exists
    // but the craft activation is not wired.
}

fn etb_dig_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: oracle puts the chosen land onto the battlefield TAPPED;
    // DigTopN can only put the chosen card into hand.
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 6,
        filter: Some(ObjectFilter {
            types: Some(TypeLine::LAND.into()),
            ..ObjectFilter::default()
        }),
        rest: DigRest::BottomRandom,
    }]
}
