//! Undead Alchemist — `{3}{U}` 4/2 Zombie.
//!
//! * "If a Zombie you control would deal combat damage to a player,
//!   instead that player mills that many cards." — a combat-damage
//!   replacement effect; no replacement primitive available in this card
//!   class, so GAP'd. (The `Mill` Scryfall keyword is an ability word,
//!   not a supported KeywordAbility variant — `keywords` stays empty.)
//! * "Whenever a creature card is put into an opponent's graveyard from
//!   their library, exile that card and create a 2/2 black Zombie
//!   creature token." — wired as a library→graveyard ZoneChange
//!   restricted to creatures an opponent controls.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undead Alchemist");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    // "a creature card ... into an opponent's graveyard from their
    // library" — the moving card is a creature owned/controlled by an
    // opponent.
    let from_library_filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: from_library_filter,
                from: Some(Zone::Library(0)),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: exile_and_make_zombie,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exile_and_make_zombie(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(zombie) = reg.interner().lookup("Zombie") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    let mut effects = Vec::new();
    if let Some(card) = trig.dying_object() {
        effects.push(Effect::ExileFromGraveyard { target: card });
    }
    effects.push(Effect::CreateToken {
        controller: trig.controller,
        token,
    });
    effects
}
