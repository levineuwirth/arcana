//! Duchess, Wayward Tavernkeep — `{3}{R}` 4/3 Legendary Human Citizen (red).
//!
//! * "Hunters for Hire — Whenever a creature you control deals combat damage
//!   to a player, put a quest counter on it." → a `DamageDealt` combat
//!   trigger whose source is a creature you control. GAP (effect): there is
//!   no damage-source accessor at resolution, so "it" (the damaging creature)
//!   can't be identified (cf. Quilled Greatwurm). The trigger is wired; its
//!   effect is GAP'd.
//! * "{1}, Remove a quest counter from a permanent you control: Create a Junk
//!   token." → an activated ability. The remove-counter cost is modeled as
//!   `remove_self_counter` (removes a quest counter from this source; the
//!   "from a permanent you control" chosen-source form has no cost field —
//!   fidelity narrowing). The Junk token is minted as a bare colorless
//!   artifact token. GAP: Junk's printed activated ability ("{T}, Sacrifice:
//!   exile top card, you may play it this turn") can't be authored on a token
//!   definition.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duchess, Wayward Tavernkeep");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let _junk = reg.interner_mut().intern("Junk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_quest_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Remove a quest counter from a permanent you \
                       control: Create a Junk token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::Quest, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_junk,
            }),
    )
}

fn combat_damage_quest_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a quest counter on it" — no damage-source accessor identifies
    // the damaging creature at resolution.
    Vec::new()
}

fn make_junk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let junk = reg.interner().lookup("Junk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(junk);
    // GAP: Junk's printed activated ability ("{T}, Sacrifice this token:
    // exile the top card of your library, you may play it this turn") is not
    // expressible on a token definition.
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: junk,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
