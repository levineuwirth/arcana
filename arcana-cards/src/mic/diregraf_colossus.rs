//! Diregraf Colossus — `{2}{B}` 2/2 Zombie Giant.
//!
//! Oracle:
//! * "This creature enters with a +1/+1 counter on it for each Zombie card
//!   in your graveyard." — ETB-counters trigger; the amount is the count of
//!   Zombie CARDS in the controller's graveyard, which the available
//!   `script::` helpers cannot compute (only `graveyard_size`, untyped). Per
//!   the dynamic-amount rule, the whole effect is GAP'd rather than emit a
//!   wrong literal.
//! * "Whenever you cast a Zombie spell, create a tapped 2/2 black Zombie
//!   creature token." — a filtered SpellCast trigger minting a 2/2 Zombie
//!   token. (No "enters tapped" token effect exists; the plain token is
//!   created untapped — a documented fidelity gap.)

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Diregraf Colossus");
    let zombie = reg.interner_mut().intern("Zombie");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let zombie_spell = script::subtype_filter(reg, "Zombie");

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "enters with a +1/+1 counter for each Zombie card in your
            // graveyard" — no script helper counts subtype-filtered graveyard
            // cards, so the dynamic counter amount is not computable.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(zombie_spell),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_zombie_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_zombie_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Zombie").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(z) = reg.interner().lookup("Zombie") {
        subtypes.0.insert(z);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
