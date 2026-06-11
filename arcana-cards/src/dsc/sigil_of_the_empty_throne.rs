//! Sigil of the Empty Throne — `{3}{W}{W}` enchantment (Conflux).
//! "Whenever you cast an enchantment spell, create a 4/4 white Angel
//! creature token with flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Sigil of the Empty Throne");
    // Pre-intern the token's name/subtype for resolver lookup.
    let _angel = reg.interner_mut().intern("Angel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ENCHANTMENT.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_angel,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 4/4 white Angel creature token with flying."
fn make_angel(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Angel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Angel") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
