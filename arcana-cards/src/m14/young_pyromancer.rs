//! Young Pyromancer — Magic 2014 uncommon. `{1}{R}` 2/1 Human
//! Shaman. "Whenever you cast an instant or sorcery spell, create a
//! 1/1 red Elemental creature token." The seed's touchstone for the
//! cast-trigger + token-creation intersection — exercises the trigger
//! `EffectFn`'s `&CardRegistry` parameter (needed to intern the token
//! subtype at resolve time) and the `SpellCast` condition with an
//! [`ObjectFilter::types_any`] OR-mask for the "instant or sorcery"
//! disjunction.
//!
//! # Rules references
//!
//! * CR 603 — triggered abilities fire on matching events; Young
//!   Pyromancer's trigger listens for [`GameEvent::SpellCast`]
//!   events filtered to "you" as caster and to instant/sorcery
//!   spells.
//! * CR 111.4 — a token's characteristics are defined by the effect
//!   that creates it.
//! * CR 704.5d — tokens in any non-battlefield zone cease to exist
//!   on the next SBA pass (shared with Servo Exhibition's path).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Young Pyromancer");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    // Interned at register so the trigger's resolver can look it up
    // via the non-mut interner at resolve time.
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine(
                            TypeLine::INSTANT | TypeLine::SORCERY)),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: create_elemental_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn create_elemental_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // Gate on the triggering event to guard against unexpected
    // reuse — the trigger condition already filtered to SpellCast +
    // instant/sorcery + you-cast, so this is belt-and-suspenders.
    match trig.trigger_event {
        GameEvent::SpellCast { .. } => {
            vec![Effect::CreateToken { controller: trig.controller, token }]
        }
        _ => Vec::new(),
    }
}
