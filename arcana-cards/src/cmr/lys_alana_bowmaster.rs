//! Lys Alana Bowmaster — `{2}{G}` 2/2 Elf Archer.
//!
//! "Reach
//!  Whenever you cast an Elf spell, you may have this creature deal 2
//!  damage to target creature with flying."
//!
//! Decomposition: Reach keyword + a SpellCast trigger filtered to Elf
//! spells you cast, targeting a creature with flying for 2 damage. The
//! "you may" is a resolution-time choice and is left implicit (the
//! engine still requires a legal target for the trigger to land).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lys Alana Bowmaster");
    let elf = reg.interner_mut().intern("Elf");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(archer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };
    let elf_spell_filter = ObjectFilter::default().with_subtype_sym(elf);
    let flyer = TargetFilter::Permanent(
        ObjectFilter::creature().with_keyword(KeywordAbility::Flying),
    );
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(elf_spell_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: deal_two_to_flyer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: flyer,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn deal_two_to_flyer(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Object(*id),
        amount: 2,
        source: trig.source,
    }]
}
