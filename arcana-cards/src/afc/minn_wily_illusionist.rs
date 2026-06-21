//! Minn, Wily Illusionist — `{1}{U}{U}` Legendary 1/3 Gnome Wizard.
//! "Whenever you draw your second card each turn, create a 1/1 blue
//!  Illusion creature token with 'This token gets +1/+0 for each other
//!  Illusion you control.'
//!  Whenever an Illusion you control dies, you may put a permanent card
//!  with mana value less than or equal to that creature's power from
//!  your hand onto the battlefield."
//!
//! First trigger: a per-draw `CardDrawn{You}` gated by an intervening-if
//! that fires only when this is the SECOND card drawn this turn. The
//! created token's internal static ("+1/+0 for each other Illusion you
//! control") is a continuous static on a TOKEN — TokenDefinition.abilities
//! only holds triggered abilities, so the static is GAP'd; a plain 1/1
//! blue Illusion is minted. Second trigger: an Illusion-dies trigger
//! that puts a permanent card from hand whose mv ≤ the dead creature's
//! power onto the battlefield.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minn, Wily Illusionist");
    let gnome = reg.interner_mut().intern("Gnome");
    let wizard = reg.interner_mut().intern("Wizard");
    let _illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let illusion_filter = script::subtype_filter(reg, "Illusion")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_draw),
                effect: make_illusion_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: illusion_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_illusion_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_second_draw(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // The trigger fires as the draw is logged, so "your second card
    // this turn" is exactly when the running count equals 2.
    script::cards_drawn_this_turn(s, you) == 2
}

fn make_illusion_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let illusion = reg.interner().lookup("Illusion").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    // GAP: token's internal static "gets +1/+0 for each other Illusion you control" — TokenDefinition.abilities holds only triggered abilities; a continuous static P/T modifier on a token is unexpressible.
    let token = TokenDefinition {
        name: illusion,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn on_illusion_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dead = trig.dying_object().unwrap_or(trig.source);
    let power = script::power_of(state, dead).max(0) as u32;
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new()
            .without_types(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
            .with_max_cmc(power),
        tapped: false,
    }]
}
