//! Joshua, Phoenix's Dominant // Phoenix, Warden of Fire
//!
//! Front: {1}{R}{W} Legendary Creature — Human Noble Wizard 3/4
//! When Joshua enters, discard up to two cards, then draw that many cards.
//! {3}{R}{W}, {T}: Exile Joshua, then return it to the battlefield transformed
//! under its owner's control. Activate only as a sorcery.
//! GAP: "discard up to two cards, then draw that many cards" (the draw count
//! matches what was discarded) — dynamic matching count not expressible.
//! Approximated as Discard 2 then Draw 2.
//! GAP: The activated ability "{3}{R}{W}, {T}: Exile Joshua, then return it
//! to the battlefield transformed" requires ExileSelf + ReturnTransformed
//! which is not a documented Effect variant; omitted.
//!
//! Back: Legendary Enchantment Creature — Saga Phoenix
//! Flying, lifelink.
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I, II — Rising Flames — Phoenix deals 2 damage to each opponent.
//! III — Flames of Rebirth — Return any number of target creature cards with
//!       total mana value 6 or less from your graveyard to the battlefield.
//!       Exile Phoenix, then return it to the battlefield (front face up).
//! GAP: back face's own Saga chapter triggers not modeled (triggered abilities
//! live on the CardDefinition, not the face).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joshua, Phoenix's Dominant");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(noble_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Phoenix, Warden of Fire");
    let saga_sub = reg.interner_mut().intern("Saga");
    let phoenix_sub = reg.interner_mut().intern("Phoenix");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    back_subtypes.0.insert(phoenix_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
            // GAP: back face's Saga chapter I/II (Rising Flames — deal 2 damage to
            // each opponent) and III (Flames of Rebirth — return creature cards from
            // graveyard + exile/return self) are back-face-only triggered abilities
            // and are not modeled here.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB trigger: discard up to two cards, then draw that many
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: activated ability "{3}{R}{W}, {T}: Exile Joshua, then return it
            // to the battlefield transformed" — ExileSelf + ReturnTransformed is not
            // a documented Effect variant; omitted.
    )
}

fn etb_loot(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to two cards, then draw that many cards" —
    // the dynamic matching draw count is not precisely expressible.
    // Approximated as: discard 2 (controller chooses), then draw 2.
    vec![
        Effect::Discard {
            player: trig.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
    ]
}
