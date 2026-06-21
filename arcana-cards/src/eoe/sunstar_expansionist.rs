//! Sunstar Expansionist — `{1}{W}` 2/3 Human Knight.
//!
//! * When this creature enters, if an opponent controls more lands than
//!   you, create a Lander token.  (The land-count comparison is computed
//!   at resolution via `script::count_matching`; the Lander token's
//!   activated ability — fetch a basic land — is GAP'd, a bare artifact
//!   token is minted.)
//! * Landfall — Whenever a land you control enters, this creature gets
//!   +1/+0 until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunstar Expansionist");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: maybe_make_lander,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_make_lander(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let land = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let mine = script::count_matching(
        state,
        &land.clone().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let theirs = script::count_matching(
        state,
        &land.controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    if theirs <= mine {
        return Vec::new();
    }
    let lander = reg.interner().lookup("Lander").unwrap_or_default();
    // GAP: the Lander token's "{2}, {T}, Sacrifice: search for a basic land"
    // activated ability is not wired on a hand-rolled token — a bare
    // colorless artifact token is minted.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: lander,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn landfall_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
