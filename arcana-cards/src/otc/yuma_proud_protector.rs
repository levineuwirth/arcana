//! Yuma, Proud Protector — `{5}{R}{G}{W}` 6/6 Legendary Creature —
//! Human Ranger.
//! "This spell costs {1} less to cast for each land card in your
//!   graveyard."
//! "Whenever Yuma enters or attacks, you may sacrifice a land. If you do,
//!   draw a card."
//! "Whenever a Desert card is put into your graveyard from anywhere,
//!   create a 4/2 green Plant Warrior creature token with reach."
//!
//! The cost-reduction static has no cost-reduction primitive and is
//! GAP'd. The "enters or attacks" ability is split into two triggers, but
//! its "you may sacrifice a land. If you do, draw a card." payload is not
//! expressible (OptionalPayment supports only Mana / Life costs, not a
//! sacrifice gate) — GAP'd. The Desert-to-graveyard trigger mints the
//! token.

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
    let name = reg.interner_mut().intern("Yuma, Proud Protector");
    let human = reg.interner_mut().intern("Human");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ranger);

    // Also intern the token's subtypes so the resolver can look them up,
    // and "Desert" so the graveyard-trigger filter resolves to a symbol.
    let _plant = reg.interner_mut().intern("Plant");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _desert = reg.interner_mut().intern("Desert");

    let desert_filter =
        script::subtype_filter(reg, "Desert").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    // GAP: static — "This spell costs {1} less to cast for each land card
    // in your graveyard." No cost-reduction primitive available.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_or_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: enters_or_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: desert_filter,
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: make_plant_warrior,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn enters_or_attacks(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice a land. If you do, draw a card."
    // OptionalPayment supports only Mana / Life costs — a sacrifice-a-land
    // gate is not expressible.
    Vec::new()
}

fn make_plant_warrior(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let plant = reg.interner().lookup("Plant").unwrap_or_default();
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(warrior);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: plant,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Reach],
            abilities: vec![],
        },
    }]
}
