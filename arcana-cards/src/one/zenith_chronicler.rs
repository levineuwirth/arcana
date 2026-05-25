//! Zenith Chronicler — `{2}` 3/1 colorless Artifact Creature — Phyrexian Construct.
//! "Whenever a player casts their first multicolored spell each turn, each other player
//! draws a card."
//! GAP: no filter for "multicolored" spells in ObjectFilter/SpellCast. Using SpellCast
//! with no filter and OncePerTurn as approximation; "each other player" uses all_players.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Zenith Chronicler");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: no multicolored filter; using SpellCast Any as approximation.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: multicolor_spell_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::OncePerTurn,
                target_requirements: Vec::new(),
            }),
    )
}

fn multicolor_spell_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let caster = trig.triggering_caster().unwrap_or(trig.controller);
    let all = script::all_players(state);
    let mut effects = Vec::new();
    for p in all {
        if p != caster {
            effects.push(Effect::DrawCards { player: p, count: 1 });
        }
    }
    effects
}
