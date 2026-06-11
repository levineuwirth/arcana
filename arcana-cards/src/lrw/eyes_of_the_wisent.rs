//! Eyes of the Wisent — `{1}{G}` Kindred Enchantment — Elemental
//! (Morningtide, 2008). "Whenever an opponent casts a blue spell
//! during your turn, you may create a 4/4 green Elemental creature
//! token."
//!
//! Wired on a color-filtered opponent `SpellCast`. GAPs: the Kindred
//! card type (not a TypeLine const) and the "during your turn"
//! restriction (no turn-ownership gate on SpellCast) — the trigger
//! over-fires on opponents' blue spells on any turn. The "may" is
//! resolved as creating the token.

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
    let name = reg.interner_mut().intern("Eyes of the Wisent");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        // GAP: the Kindred card type is not a TypeLine const; only the
        // Enchantment bit (plus the Elemental subtype) is recorded.
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: "during your turn" — SpellCast has no turn-ownership
                // restriction; this fires on opponents' blue spells on any
                // turn.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new().with_colors(ColorSet::blue()),
                    ),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: wisent_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may create a 4/4 green Elemental creature token." ("may"
/// resolved as creating it.)
fn wisent_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: elemental,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
