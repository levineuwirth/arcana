//! Sedgemoor Witch — `{2}{B}` 3/2 Human Warlock. Menace; Ward—Pay 3 life
//! (non-mana ward, not expressible → GAP). Magecraft: whenever you cast or
//! copy an instant or sorcery spell, create a 1/1 black-green Pest with
//! "When this token dies, you gain 1 life".

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
    let name = reg.interner_mut().intern("Sedgemoor Witch");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ward—Pay 3 life is a non-mana ward, not expressible.
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Magecraft: cast OR copy. The engine's SpellCast catches
                // the cast side (copy side is a documented fidelity gap).
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    ))),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_pest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_pest(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pest = reg.interner().lookup("Pest").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    // GAP: the Pest's own "When this token dies, you gain 1 life" ability
    // is not expressible via the TokenDefinition surface shown.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: pest,
            colors: ColorSet::black() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
