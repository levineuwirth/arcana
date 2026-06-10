//! Genesis Chamber — `{2}` artifact (Darksteel, 2004).
//! "Whenever a nontoken creature enters, if this artifact is untapped,
//! that creature's controller creates a 1/1 colorless Myr artifact
//! creature token." ZoneChange trigger over nontoken creatures entering
//! the battlefield; the token goes to the entering creature's controller.
//! GAP: the intervening-if "if this artifact is untapped" has no
//! conditions:: predicate (no source-untapped helper) — the trigger fires
//! unconditionally.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genesis Chamber");
    let _myr = reg.interner_mut().intern("Myr");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().nontoken(),
                    from: None,
                    to: Zone::Battlefield,
                },
                // GAP: intervening-if "if this artifact is untapped" — no
                // source-untapped conditions:: predicate available.
                intervening_if: None,
                effect: make_myr,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn make_myr(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let entered = trig.entering_object().unwrap_or(trig.source);
    let controller = script::target_controller(state, entered, trig.controller);
    let myr = reg.interner().lookup("Myr").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    vec![Effect::CreateToken {
        controller,
        token: TokenDefinition {
            name: myr,
            colors: ColorSet::new(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
