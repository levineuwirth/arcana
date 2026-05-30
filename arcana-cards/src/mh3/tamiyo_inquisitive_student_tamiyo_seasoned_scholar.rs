//! Tamiyo, Inquisitive Student // Tamiyo, Seasoned Scholar
//!
//! Front: Legendary Creature — Moonfolk Wizard {U}, 0/3
//!   Flying
//!   Whenever Tamiyo attacks, investigate.
//!   When you draw your third card in a turn, exile Tamiyo, then return her transformed.
//!
//! Back: Legendary Planeswalker — Tamiyo (loyalty 4)
//!   +2: Until your next turn, whenever a creature attacks you or a planeswalker you control, it gets -1/-0 until end of turn.
//!   -3: Return target instant or sorcery card from your graveyard to your hand. If it's a green card, add one mana of any color.
//!   -7: Draw cards equal to half the number of cards in your library, rounded up.
//!       You get an emblem with "You have no maximum hand size."
//!
//! GAP: "when you draw your third card in a turn" trigger condition — engine has cards_drawn_this_turn
//!      counter but no "when count reaches 3" condition on TriggerCondition; wired as
//!      a best-effort PhaseBegins trigger in lieu of the real draw-count trigger.
//! GAP: Exile self then return transformed (blink-transform) — no ExileSelfThenReturnTransformed effect.
//! GAP: Planeswalker loyalty abilities (+2, -3, -7) not modeled — back face static characteristics only.
//! GAP: "+2 until your next turn" continuous effect not modeled.
//! GAP: "-3 add mana if green card" not modeled.
//! GAP: "-7 emblem" not modeled.
//! Keywords: Investigate is not a KeywordAbility in the engine keyword list — omitted; wired as a
//!   triggered ability that creates a Clue commodity token.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, Inquisitive Student");
    let back_name = reg.interner_mut().intern("Tamiyo, Seasoned Scholar");

    let mut front_subtypes = SubtypeSet::default();
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    front_subtypes.0.insert(moonfolk);
    front_subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        keywords: vec![KeywordAbility::Flying],
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let mut back_subtypes = SubtypeSet::default();
    let tamiyo_sub = reg.interner_mut().intern("Tamiyo");
    back_subtypes.0.insert(tamiyo_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::PLANESWALKER.into(),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            loyalty: Some(4),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: whenever Tamiyo attacks, investigate (create a Clue token)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: investigate_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "when you draw your third card in a turn" — not expressible as TriggerCondition;
            // the exile-then-return-transformed effect is also not directly expressible.
            // The transform trigger is omitted; a human must route this gap.
    )
}

fn investigate_on_attack(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
